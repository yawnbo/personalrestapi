// these are pretty basic scripts and won't be used anywhere else so it's not worth starting them
// as a service due to how independent they are
//
// FIXME: the errors in this file do NOT use the appresult errs
use axum::{
    Router,
    extract::Query,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
    routing::get,
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use serde::Deserialize;
use tracing::{debug, error, info};

use crate::server::{
    extractors::RequiredAuthentication, services::Services, utils::signature_utils::SignatureUtil,
};

#[derive(Deserialize)]
struct ProxyQuery {
    url: String,
    schema: Option<String>,
}

pub struct ProxyController;

impl ProxyController {
    pub fn app() -> Router {
        Router::new()
            .route("/", get(Self::proxy_get).options(Self::proxy_options))
            .route("/captions", get(Self::proxy_captions))
    }

    async fn proxy_get(
        RequiredAuthentication(user_id, services): RequiredAuthentication,
        Query(params): Query<ProxyQuery>,
        headers: HeaderMap,
    ) -> Result<Response, (StatusCode, String)> {
        let target_url = Self::decode_url(&params.url)?;

        if !target_url.starts_with("http://") && !target_url.starts_with("https://") {
            return Err((StatusCode::BAD_REQUEST, "Invalid URL format".to_string()));
        }

        let schema = params.schema.as_deref().unwrap_or("sports");
        debug!("Proxying (schema={}): {}", schema, target_url);

        let client = reqwest::Client::new();
        let request_builder =
            Self::apply_schema_headers(client.get(&target_url), schema, &target_url, &headers);
        debug!("Sending request to target");

        let target_response = request_builder.send().await.map_err(|e| {
            error!("Request failed: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e))
        })?;

        debug!(
            "Received response with status: {}",
            target_response.status()
        );

        // this line gets hit a good amount for some reason and causes soft errors downstream but
        // they recover when calling the playlist again for some reason, might need to look into it
        //
        // in specific i get 520s from the servers which is weird
        let response_status = target_response.status();
        if !response_status.is_success() {
            let target_bytes = target_response.bytes().await.map_or_else(
                |_| "No response".to_string(),
                |b| {
                    String::from_utf8(b.to_vec())
                        .unwrap_or_else(|_| "Non-UTF8 response".to_string())
                },
            );
            error!("Response from target not successful: {}", target_bytes);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Received fatal status code: {}", response_status),
            ));
        }

        let content_type = target_response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let content_encoding = target_response
            .headers()
            .get(header::CONTENT_ENCODING)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let response_status = target_response.status();
        let content_range = target_response
            .headers()
            .get(header::CONTENT_RANGE)
            .cloned();
        let accept_ranges = target_response
            .headers()
            .get(header::ACCEPT_RANGES)
            .cloned();

        let is_mp4 = content_type.contains("video/mp4");
        debug!(
            "Content-Type: {}, Encoding: {:?}, Is MP4: {}",
            content_type, content_encoding, is_mp4
        );

        debug!("Reading response bytes");
        let bytes = target_response.bytes().await.map_err(|e| {
            error!("Failed to read response: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e))
        })?;
        debug!("Read {} bytes", bytes.len());

        let decompressed = if content_encoding.as_deref() == Some("zstd") {
            debug!("Decompressing zstd-encoded response");
            zstd::decode_all(&bytes[..]).map_err(|e| {
                error!("Failed to decompress zstd: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to decompress response".to_string(),
                )
            })?
        } else {
            bytes.to_vec()
        };
        debug!("Decompressed size: {} bytes", decompressed.len());

        // check if content starts with #EXT to detect M3U8, or default to M3U8 unless MP4
        let is_m3u8 = if is_mp4 {
            false
        } else {
            decompressed.starts_with(b"#EXT")
                || content_type.contains("mpegurl")
                || content_type.contains("m3u8")
        };
        debug!("Detected as M3U8: {}, MP4: {}", is_m3u8, is_mp4);

        if is_m3u8 {
            debug!("Processing as M3U8 playlist");
            let text = String::from_utf8(decompressed).map_err(|e| {
                error!("Failed to parse m3u8 as UTF-8: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Invalid m3u8 encoding".to_string(),
                )
            })?;
            debug!("M3U8 text length: {} chars", text.len());

            let processed_body = Self::process_m3u8_by_schema_with_retry(
                &text,
                &target_url,
                &user_id,
                &services,
                schema,
            )?;
            debug!(
                "Processed M3U8, response length: {} bytes",
                processed_body.len()
            );

            let mut response_headers = HeaderMap::new();
            response_headers.insert(
                header::CONTENT_TYPE,
                "application/vnd.apple.mpegurl"
                    .parse()
                    .expect("Static header value should parse"),
            );
            response_headers.insert(
                header::CACHE_CONTROL,
                "no-cache"
                    .parse()
                    .expect("Static header value should parse"),
            );

            Ok((StatusCode::OK, response_headers, processed_body).into_response())
        } else {
            let bytes = decompressed;

            let mut response_headers = HeaderMap::new();
            // rare cases occur where the content is an mp4 i think but i haven't found one yet so
            // im going to default to mp2t for now to see if errors are resolved
            // let default_content_type = if is_mp4 { "video/mp4" } else { "video/mp2t" };
            //
            // note that the returned types are some random js, css, img or other extensions but
            // they're just mp2t packets

            response_headers.insert(
                header::CONTENT_TYPE,
                "video/mp2t"
                    .parse()
                    .expect("Static header value should parse"), // content_type
                                                                 //     .parse()
                                                                 //     .unwrap_or_else(|_| default_content_type.parse().unwrap()),
            );

            let cache_control = if is_mp4 {
                "public, max-age=3600"
            } else {
                "public, max-age=31536000"
            };

            response_headers.insert(
                header::CACHE_CONTROL,
                cache_control
                    .parse()
                    .expect("Static header value should parse"),
            );
            response_headers.insert(
                header::CONTENT_LENGTH,
                bytes
                    .len()
                    .to_string()
                    .parse()
                    .expect("Content length should parse"),
            );

            // Range headers need to be forwarded i think
            if let Some(range) = content_range {
                response_headers.insert(header::CONTENT_RANGE, range);
            }

            if let Some(accept) = accept_ranges {
                response_headers.insert(header::ACCEPT_RANGES, accept);
            }

            Ok((
                StatusCode::from_u16(response_status.as_u16()).unwrap_or(StatusCode::OK),
                response_headers,
                bytes,
            )
                .into_response())
        }
    }

    async fn proxy_options() -> impl IntoResponse {
        StatusCode::NO_CONTENT
    }

    // this is unathenticated because of a previous track implementation but i think it's fetched
    // with the auth header anyways now so it should be closed again - double check later TODO:
    //
    // ps: im not checking this i don't care steal my captions dude
    async fn proxy_captions(
        Query(params): Query<ProxyQuery>,
    ) -> Result<Response, (StatusCode, String)> {
        let target_url = Self::decode_url(&params.url)?;

        if !target_url.starts_with("http://") && !target_url.starts_with("https://") {
            return Err((StatusCode::BAD_REQUEST, "Invalid URL format".to_string()));
        }

        debug!("Proxying caption: {}", target_url);

        let client = reqwest::Client::new();
        let request_builder = client
            .get(&target_url)
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:145.0) Gecko/20100101 Firefox/145.0",
            )
            .header(header::ACCEPT, "*/*");

        let target_response = request_builder.send().await.map_err(|e| {
            error!("Caption request failed: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e))
        })?;

        if !target_response.status().is_success() {
            error!("Caption fetch failed: {}", target_response.status());
            return Err((
                StatusCode::from_u16(target_response.status().as_u16())
                    .unwrap_or(StatusCode::BAD_GATEWAY),
                format!("Failed to fetch caption: {}", target_response.status()),
            ));
        }

        let content_type = target_response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("text/vtt")
            .to_string();

        let bytes = target_response.bytes().await.map_err(|e| {
            error!("Failed to read caption response: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e))
        })?;

        let mut response_headers = HeaderMap::new();
        response_headers.insert(
            header::CONTENT_TYPE,
            content_type.parse().unwrap_or_else(|_| {
                "text/vtt"
                    .parse()
                    .expect("Static header value should parse")
            }),
        );
        response_headers.insert(
            header::CACHE_CONTROL,
            "public, max-age=86400"
                .parse()
                .expect("Static header value should parse"),
        );
        response_headers.insert(
            header::CONTENT_LENGTH,
            bytes
                .len()
                .to_string()
                .parse()
                .expect("Content length should parse"),
        );

        Ok((StatusCode::OK, response_headers, bytes).into_response())
    }

    fn decode_url(url_param: &str) -> Result<String, (StatusCode, String)> {
        if url_param.starts_with("http://") || url_param.starts_with("https://") {
            urlencoding::decode(url_param)
                .map(|s| s.to_string())
                .map_err(|e| {
                    error!("Failed to decode URL: {}", e);
                    (StatusCode::BAD_REQUEST, "Invalid URL encoding".to_string())
                })
        } else {
            let mut padded = url_param.to_string();
            while padded.len() % 4 != 0 {
                padded.push('=');
            }

            URL_SAFE
                .decode(&padded)
                .map_err(|e| {
                    error!("Failed to decode base64: {}", e);
                    (StatusCode::BAD_REQUEST, "Invalid URL encoding".to_string())
                })
                .and_then(|bytes| {
                    String::from_utf8(bytes).map_err(|e| {
                        error!("Failed to parse UTF-8: {}", e);
                        (StatusCode::BAD_REQUEST, "Invalid URL encoding".to_string())
                    })
                })
        }
    }

    fn apply_schema_headers(
        mut request_builder: reqwest::RequestBuilder,
        schema: &str,
        target_url: &str,
        headers: &HeaderMap,
    ) -> reqwest::RequestBuilder {
        match schema {
            "movie" => {
                request_builder
                    .header(header::HOST, "storm.vodvidl.site")
                    .header(header::ORIGIN, "https://vidlink.pro")
                    .header(header::REFERER, "https://vidlink.pro/")
                    .header(
                        header::USER_AGENT,
                        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:145.0) Gecko/20100101 Firefox/145.0",
                    )
                    .header(header::ACCEPT, "*/*")
            }
            "sports" => {
                if !target_url.contains("gg.poocloud.in") {
                    request_builder = request_builder
                        .header(header::ORIGIN, "https://embednow.top")
                        .header(header::REFERER, "https://embednow.top/")
                        .header(
                            header::USER_AGENT,
                            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
                        )
                        .header(header::ACCEPT, "*/*");
                } else {
                    request_builder = request_builder
                        .header(header::REFERER, "https://api.ppvs.su/api/streams/")
                        .header(header::ORIGIN, "https://api.ppvs.su/api/streams")
                        .header(
                            header::USER_AGENT,
                            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
                        )
                        .header(header::ACCEPT, "*/*");
                }

                if let Some(range_header) = headers.get(header::RANGE) {
                    if let Ok(range_value) = range_header.to_str() {
                        request_builder = request_builder.header(header::RANGE, range_value);
                    }
                }

                request_builder
            }
            "captions" => {
                request_builder
                    .header(
                        header::USER_AGENT,
                        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:145.0) Gecko/20100101 Firefox/145.0",
                    )
                    .header(header::ACCEPT, "*/*")
            }
            _ => {
                // default to sports if anything, but this ideally shouldn't happen
                info!("Unknown schema, falling back to sports headers");
                request_builder = request_builder
                    .header(header::REFERER, "https://api.ppvs.su/api/streams/")
                    .header(header::ORIGIN, "https://api.ppvs.su/api/streams")
                    .header(
                        header::USER_AGENT,
                        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
                    )
                    .header(header::ACCEPT_ENCODING, "gzip, deflate, br, zstd")
                    .header(header::ACCEPT, "*/*");

                if let Some(range_header) = headers.get(header::RANGE) {
                    if let Ok(range_value) = range_header.to_str() {
                        request_builder = request_builder.header(header::RANGE, range_value);
                    }
                }

                request_builder
            }
        }
    }

    fn process_m3u8_by_schema(
        text: &str,
        target_url: &str,
        user_id: &str,
        services: &Services,
        schema: &str,
    ) -> Result<String, (StatusCode, String)> {
        match schema {
            "movie" => {
                debug!("Processing with movie schema");
                Self::process_m3u8_movie(text, target_url, user_id, services)
            }
            _ => {
                debug!("Processing with sports schema");
                Self::process_m3u8(text, target_url, user_id, services)
            }
        }
    }

    fn process_m3u8_by_schema_with_retry(
        text: &str,
        target_url: &str,
        user_id: &str,
        services: &Services,
        schema: &str,
    ) -> Result<String, (StatusCode, String)> {
        let result = Self::process_m3u8_by_schema(text, target_url, user_id, services, schema);

        match result {
            Err((StatusCode::INTERNAL_SERVER_ERROR, ref err_msg)) => {
                error!(
                    "M3U8 processing failed with 500, retrying once: {}",
                    err_msg
                );
                // Retry once on 500 error
                Self::process_m3u8_by_schema(text, target_url, user_id, services, schema)
            }
            other => other,
        }
    }

    fn process_m3u8(
        text: &str,
        target_url: &str,
        user_id: &str,
        services: &Services,
    ) -> Result<String, (StatusCode, String)> {
        let base_url = url::Url::parse(target_url).map_err(|e| {
            error!("Failed to parse base URL: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Invalid base URL: {}", e),
            )
        })?;

        let base_path = format!(
            "{}://{}{}",
            base_url.scheme(),
            base_url.host_str().unwrap_or(""),
            &base_url.path()[..base_url.path().rfind('/').unwrap_or(0) + 1]
        );

        let lines: Vec<String> = text
            .lines()
            .filter(|line| !line.trim().starts_with("##"))
            .map(|line| {
                let trimmed = line.trim();

                if trimmed.is_empty() || trimmed.starts_with('#') {
                    return line.to_string();
                }

                let full_url = if trimmed.starts_with("http://") || trimmed.starts_with("https://")
                {
                    trimmed.to_string()
                } else {
                    match url::Url::parse(&base_path).and_then(|base| base.join(trimmed)) {
                        Ok(resolved) => resolved.to_string(),
                        Err(e) => {
                            error!("Failed to resolve: {} - {}", trimmed, e);
                            return line.to_string();
                        }
                    }
                };

                let encoded = URL_SAFE
                    .encode(full_url.as_bytes())
                    .trim_end_matches('=')
                    .to_string();

                // Generate signed URL parameters
                let expiry = SignatureUtil::generate_expiry(12);
                // Sign just the encoded URL to avoid path mismatch issues
                let signature = services
                    .signature_util
                    .generate_signature(user_id, expiry, &encoded);

                format!(
                    "/api/v1/proxy?url={}&schema=sports&sig={}&exp={}&user={}",
                    encoded,
                    signature,
                    expiry,
                    urlencoding::encode(user_id)
                )
            })
            .collect();

        Ok(lines.join("\n"))
    }

    fn process_m3u8_movie(
        text: &str,
        target_url: &str,
        user_id: &str,
        services: &Services,
    ) -> Result<String, (StatusCode, String)> {
        let base_url = url::Url::parse(target_url).map_err(|e| {
            error!("Failed to parse base URL: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Invalid base URL: {}", e),
            )
        })?;

        let base_path = format!(
            "{}://{}{}",
            base_url.scheme(),
            base_url.host_str().unwrap_or(""),
            &base_url.path()[..base_url.path().rfind('/').unwrap_or(0) + 1]
        );

        let lines: Vec<String> = text
            .lines()
            .filter(|line| !line.trim().starts_with("##"))
            .map(|line| {
                let trimmed = line.trim();

                if trimmed.is_empty() || trimmed.starts_with('#') {
                    return line.to_string();
                }

                let full_url = if trimmed.starts_with("http://") || trimmed.starts_with("https://")
                {
                    trimmed.to_string()
                } else if trimmed.starts_with('/') {
                    format!(
                        "{}://{}{}",
                        base_url.scheme(),
                        base_url.host_str().unwrap_or(""),
                        trimmed
                    )
                } else {
                    match url::Url::parse(&base_path).and_then(|base| base.join(trimmed)) {
                        Ok(resolved) => resolved.to_string(),
                        Err(e) => {
                            error!("Failed to resolve: {} - {}", trimmed, e);
                            return line.to_string();
                        }
                    }
                };

                let encoded = URL_SAFE
                    .encode(full_url.as_bytes())
                    .trim_end_matches('=')
                    .to_string();

                // Generate signed URL parameters
                let expiry = SignatureUtil::generate_expiry(12);
                // Sign just the encoded URL to avoid path mismatch issues
                let signature = services
                    .signature_util
                    .generate_signature(user_id, expiry, &encoded);

                format!(
                    "/api/v1/proxy?url={}&schema=movie&sig={}&exp={}&user={}",
                    encoded,
                    signature,
                    expiry,
                    urlencoding::encode(user_id)
                )
            })
            .collect();

        Ok(lines.join("\n"))
    }
}
