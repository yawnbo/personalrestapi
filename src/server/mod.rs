// fun file!
mod api;
pub mod dtos;
pub mod error;
pub mod extractors;
pub mod services;
pub mod utils;

use std::future::ready;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Context;
use axum::Extension;
use axum::extract::MatchedPath;
use axum::http::header::{self, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::method;
use axum::http::request::Parts as RequestParts;
use axum::http::{HeaderValue, Request};
use axum::middleware::{self, Next};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{BoxError, Json, Router, error_handling::HandleErrorLayer, http::StatusCode};
use lazy_static::lazy_static;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use serde_json::json;
use tower::{ServiceBuilder, buffer::BufferLayer, limit::RateLimitLayer};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};
use tracing::{debug, info};

use crate::config::AppConfig;
use crate::database::Database;
use crate::database::RedisDatabase;
use crate::server::services::Services;
use crate::server::services::seed_services::SeedService;
lazy_static! {
    static ref HTTP_TIMEOUT: u64 = 30;
    static ref EXPONENTIAL_SECONDS: &'static [f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];
}

pub struct ApplicationServer;

impl ApplicationServer {
    pub async fn serve(
        config: Arc<AppConfig>,
        db: Database,
        redis_db: RedisDatabase,
    ) -> anyhow::Result<()> {
        // do this however you like, i use the prometheus exporter because grafana is nice
        let recorder_handle = PrometheusBuilder::new()
            .set_buckets_for_metric(
                Matcher::Full(String::from("http_requests_duration_seconds")),
                *EXPONENTIAL_SECONDS,
            )
            .context("could not setup metric buckets, what happened to my EXPONENTIAL_SECONDS?")?
            .install_recorder()
            .context("i can't run the metric recorder yo")?;

        let services = Services::new(db, redis_db, config.clone());

        if config.seed {
            info!("seeding enabled, creating test data...");
            SeedService::new(services.clone())
                .seed()
                .await
                .expect("couldn't seed the db");
        }

        // the cors configs are independent to the proxy and general api layers but they can really be combined
        // if needed and it's very easy to do so
        let cors_origin = config.cors_origin.clone();
        let preview_cors_origin = config.preview_cors_origin.clone();
        let cors_origin_proxy = cors_origin.clone();
        let preview_cors_origin_proxy = preview_cors_origin.clone();

        let cors = CorsLayer::new()
            .allow_origin(AllowOrigin::predicate(
                move |origin: &HeaderValue, _request_parts: &RequestParts| {
                    let origin_str = origin.to_str().unwrap_or("");

                    if preview_cors_origin == "*" {
                        return true;
                    }

                    if let Some(host) = origin_str
                        .strip_prefix("https://")
                        .or_else(|| origin_str.strip_prefix("http://"))
                    {
                        if host == &preview_cors_origin[..] || host.ends_with(&preview_cors_origin)
                        {
                            return true;
                        }

                        if host == &cors_origin[..] || host.ends_with(&cors_origin) {
                            return true;
                        }

                        // patch to allow for new domains
                        // its important to note that this might as well be a simple backdoor if
                        // you are planning on using this, so please change it
                        //
                        // what words do i even put here so someone stumbles into this FIXME: TODO:
                        if host == "yawnbo.com" || host.ends_with("yawnbo.com") {
                            return true;
                        }
                    }

                    if origin_str == &preview_cors_origin || origin_str == &cors_origin {
                        return true;
                    }

                    false
                },
            ))
            .allow_methods([
                method::Method::GET,
                method::Method::POST,
                method::Method::PUT,
                method::Method::DELETE,
                method::Method::OPTIONS,
            ])
            // this would be chill if it was ANY but for some reason tower doesn't like it (maybe
            // its fine now because i changed other things but i don't want to test it)
            .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT])
            .allow_credentials(true);

        let proxy_cors = CorsLayer::new()
            .allow_origin(AllowOrigin::predicate(
                move |origin: &HeaderValue, _request_parts: &RequestParts| {
                    let origin_str = origin.to_str().unwrap_or("");

                    if preview_cors_origin_proxy == "*" {
                        return true;
                    }

                    if let Some(host) = origin_str
                        .strip_prefix("https://")
                        .or_else(|| origin_str.strip_prefix("http://"))
                    {
                        if host == &preview_cors_origin_proxy[..]
                            || host.ends_with(&preview_cors_origin_proxy)
                        {
                            return true;
                        }

                        if host == &cors_origin_proxy[..] || host.ends_with(&cors_origin_proxy) {
                            return true;
                        }
                    }

                    if origin_str == &preview_cors_origin_proxy || origin_str == &cors_origin_proxy
                    {
                        return true;
                    }

                    false
                },
            ))
            .allow_methods([method::Method::GET, method::Method::OPTIONS])
            .allow_headers([AUTHORIZATION, CONTENT_TYPE, header::RANGE])
            .expose_headers([header::CONTENT_LENGTH, header::CONTENT_RANGE])
            .allow_credentials(true);

        // looks kind of messy but its just make routes that are protected with general cors
        // then merging them with the proxy routes
        let api_routes = Router::new()
            .nest("/streams", api::stream_controller::StreamController::app())
            .nest("/users", api::user_controller::UserController::app())
            .nest("/movies", api::movie_controller::MovieController::app())
            .route("/health", get(api::health))
            .layer(cors);

        let proxy_routes = Router::new()
            .nest("/proxy", api::proxy_controller::ProxyController::app())
            .layer(proxy_cors);

        let router = Router::new()
            .nest("/api/v1", api_routes.merge(proxy_routes))
            .route("/", get(api::health))
            .route("/metrics", get(move || ready(recorder_handle.render())))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(HandleErrorLayer::new(Self::handle_timeout_error))
                    .timeout(Duration::from_secs(*HTTP_TIMEOUT))
                    .layer(Extension(services))
                    .layer(BufferLayer::new(1024))
                    .layer(RateLimitLayer::new(5, Duration::from_secs(1))),
            )
            .route_layer(middleware::from_fn(Self::track_metrics));

        let router = router.fallback(Self::handle_404);

        let port = format!("0.0.0.0:{}", config.port);
        let addr = tokio::net::TcpListener::bind(&port).await.unwrap();

        info!("Setup completed, initialized server on port {port}");
        debug!("routes initialized, listening on port {}", &port);

        axum::serve(addr, router)
            .with_graceful_shutdown(Self::shutdown_signal())
            .await
            .context("axum serving failed")?;

        Ok(())
    }

    // custom timeout layer
    async fn handle_timeout_error(err: BoxError) -> (StatusCode, Json<serde_json::Value>) {
        if err.is::<tower::timeout::error::Elapsed>() {
            (
                StatusCode::REQUEST_TIMEOUT,
                Json(json!({
                    "error":
                        format!(
                            "request took longer than the configured {} second timeout",
                            *HTTP_TIMEOUT
                        )
                })),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("unhandled internal error: {}", err)
                })),
            )
        }
    }

    async fn track_metrics(request: Request<axum::body::Body>, next: Next) -> impl IntoResponse {
        let path = if let Some(matched_path) = request.extensions().get::<MatchedPath>() {
            matched_path.as_str().to_owned()
        } else {
            request.uri().path().to_owned()
        };
        let start = Instant::now();
        let method = request.method().clone();
        let response = next.run(request).await;
        let latency = start.elapsed().as_secs_f64();
        let status = response.status().as_u16().to_string();

        metrics::counter!("http_requests_total", "method" => method.to_string(), "path" => path.clone(), "status" => status.clone()).increment(1);

        metrics::histogram!("http_requests_duration_seconds", "method" => method.to_string(), "path" => path, "status" => status).record(latency);

        response
    }

    async fn shutdown_signal() {
        tokio::signal::ctrl_c()
            .await
            .expect("how did we crash listening for SIGINT");
        println!("signal shutdown");
    }

    async fn handle_404() -> impl IntoResponse {
        (
            StatusCode::NOT_FOUND,
            axum::response::Json(serde_json::json!({
            "errors":{
            "message": vec!(String::from("This resource doesn't exist.")),}
            })),
        )
    }
}
