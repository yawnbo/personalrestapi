use axum::extract::{Extension, Path, Query};
use axum::routing::get;
use axum::{Json, Router};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use tracing::{error, info};

use crate::server::dtos::oidc_dto::{OidcCallbackQuery, OidcLoginResponse};
use crate::server::dtos::user_dto::UserAuthenicationResponse;
use crate::server::error::{AppResult, Error};
use crate::server::extractors::UserAgentExtractor;
use crate::server::services::Services;

pub struct OidcController;

impl OidcController {
    pub fn app() -> Router {
        Router::new()
            .route("/:provider/login", get(Self::login_endpoint))
            .route("/:provider/callback", get(Self::callback_endpoint))
            .route("/providers", get(Self::providers_endpoint))
    }

    /// GET /auth/oidc/:provider/login
    /// Initiates the OIDC login flow and returns the authorization URL
    async fn login_endpoint(
        Extension(services): Extension<Services>,
        Path(provider): Path<String>,
    ) -> AppResult<Json<OidcLoginResponse>> {
        info!("initiating OIDC login for provider: {}", provider);

        let oidc_service = services.oidc.as_ref().ok_or_else(|| {
            Error::BadRequest("OIDC authentication is not configured".to_string())
        })?;

        let response = oidc_service.get_auth_url(&provider)?;

        Ok(Json(response))
    }

    /// GET /auth/oidc/:provider/callback
    /// Handles the OIDC callback and exchanges the code for tokens
    async fn callback_endpoint(
        jar: CookieJar,
        Extension(services): Extension<Services>,
        Path(provider): Path<String>,
        Query(query): Query<OidcCallbackQuery>,
        UserAgentExtractor(user_agent): UserAgentExtractor,
    ) -> AppResult<(CookieJar, Json<UserAuthenicationResponse>)> {
        info!("handling OIDC callback for provider: {}", provider);

        // Check for OAuth errors
        if let Some(error) = query.error {
            let description = query.error_description.unwrap_or_default();
            error!("OIDC error from {}: {} - {}", provider, error, description);
            return Err(Error::BadRequest(format!(
                "Authentication failed: {}",
                description
            )));
        }

        // Extract the authorization code
        let code = query.code.ok_or_else(|| {
            Error::BadRequest("Missing authorization code".to_string())
        })?;

        let oidc_service = services.oidc.as_ref().ok_or_else(|| {
            Error::BadRequest("OIDC authentication is not configured".to_string())
        })?;

        let (user, refresh_token) = oidc_service
            .authenticate(&provider, &code, user_agent)
            .await?;

        // Set the refresh token as a cookie
        let cookie = jar.add(Cookie::new("refresh_token", refresh_token));

        Ok((cookie, Json(UserAuthenicationResponse { user })))
    }

    /// GET /auth/oidc/providers
    /// Returns a list of available OIDC providers
    async fn providers_endpoint(
        Extension(services): Extension<Services>,
    ) -> AppResult<Json<Vec<&'static str>>> {
        let oidc_service = services.oidc.as_ref().ok_or_else(|| {
            Error::BadRequest("OIDC authentication is not configured".to_string())
        })?;

        let providers = oidc_service.get_available_providers();

        Ok(Json(providers))
    }
}
