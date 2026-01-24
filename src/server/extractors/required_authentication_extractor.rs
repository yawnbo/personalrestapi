use axum::Extension;
use axum::extract::{FromRequestParts, Query};
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use serde::Deserialize;
use tracing::error;

use crate::server::error::Error;
use crate::server::services::Services;

/// Query parameters for fallback authentication methods
#[derive(Deserialize)]
struct AuthQuery {
    sig: Option<String>,
    exp: Option<String>,
    user: Option<String>,
    token: Option<String>,
}

/// Combined authentication extractor that tries multiple methods in order:
/// 1. Bearer token (Authorization header)
/// 2. Signed URL (query params: sig, exp, user, url)
/// 3. Token query param (fallback)
///
/// For single-method authentication, prefer using:
/// - `JwtAuthentication` for Bearer token only
/// - `SignedUrlAuthentication` for signed URL only
pub struct RequiredAuthentication(pub String, pub Services);

impl<S> FromRequestParts<S> for RequiredAuthentication
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(services): Extension<Services> = Extension::from_request_parts(parts, state)
            .await
            .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

        // Try Bearer token first
        if let Some(authorization_header) = parts.headers.get(AUTHORIZATION) {
            if let Ok(header_value) = authorization_header.to_str() {
                if let Some(token) = header_value.strip_prefix("Bearer ") {
                    if !token.is_empty() {
                        let user_id = services
                            .jwt_util
                            .get_user_id_from_token(token.to_string())
                            .map_err(|err| {
                                error!("could not validate user ID from token: {:?}", err);
                                Error::Unauthorized
                            })?;
                        return Ok(RequiredAuthentication(user_id, services));
                    }
                }
            }
        }

        // Try query params
        let Query(query): Query<AuthQuery> = Query::from_request_parts(parts, state)
            .await
            .map_err(|_| Error::Unauthorized)?;

        // Try signed URL
        if let (Some(sig), Some(exp_str), Some(user)) =
            (query.sig.as_ref(), query.exp.as_ref(), query.user.as_ref())
        {
            let expiry = exp_str.parse::<i64>().map_err(|_| {
                error!("invalid expiry timestamp");
                Error::Unauthorized
            })?;

            // Extract url parameter from the RAW query string (before URL decoding)
            let url_param = parts
                .uri
                .query()
                .and_then(|q| {
                    q.split('&')
                        .find(|param| param.starts_with("url="))
                        .and_then(|param| param.strip_prefix("url="))
                })
                .ok_or_else(|| {
                    error!("missing url parameter in signed URL");
                    Error::Unauthorized
                })?;

            if services
                .signature_util
                .verify_signature(user, expiry, url_param, sig)
            {
                return Ok(RequiredAuthentication(user.clone(), services));
            } else {
                error!(
                    "Signature was invalid, url: {}, user: {}, expiry: {}, sig: {}",
                    url_param, user, expiry, sig
                );
                return Err(Error::Unauthorized);
            }
        }

        // Try token query param as fallback
        if let Some(token) = query.token {
            let user_id = services
                .jwt_util
                .get_user_id_from_token(token)
                .map_err(|err| {
                    error!("could not validate user ID from token: {:?}", err);
                    Error::Unauthorized
                })?;
            return Ok(RequiredAuthentication(user_id, services));
        }

        error!("no valid authentication method found");
        Err(Error::Unauthorized)
    }
}
