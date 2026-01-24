use axum::Extension;
use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use tracing::error;

use crate::server::error::Error;
use crate::server::services::Services;

/// Extractor for pure JWT Bearer token authentication.
/// Extracts user_id from Authorization: Bearer <token> header.
pub struct JwtAuthentication(pub String, pub Services);

impl<S> FromRequestParts<S> for JwtAuthentication
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(services): Extension<Services> = Extension::from_request_parts(parts, state)
            .await
            .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

        let authorization_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(Error::Unauthorized)?;

        let header_value = authorization_header
            .to_str()
            .map_err(|_| Error::Unauthorized)?;

        if !header_value.starts_with("Bearer ") {
            error!("request does not contain valid 'Bearer' prefix for authorization");
            return Err(Error::Unauthorized);
        }

        let token = header_value
            .strip_prefix("Bearer ")
            .ok_or(Error::Unauthorized)?;

        if token.is_empty() {
            error!("request does not contain a valid token");
            return Err(Error::Unauthorized);
        }

        let user_id = services
            .jwt_util
            .get_user_id_from_token(token.to_string())
            .map_err(|err| {
                error!("could not validate user ID from token: {:?}", err);
                Error::Unauthorized
            })?;

        Ok(JwtAuthentication(user_id, services))
    }
}
