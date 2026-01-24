use axum::Extension;
use axum::extract::{FromRequestParts, Query};
use axum::http::request::Parts;
use serde::Deserialize;
use tracing::{debug, error};

use crate::server::error::Error;
use crate::server::services::Services;

/// Query parameters for signed URL authentication
#[derive(Deserialize)]
pub struct SignedUrlQuery {
    pub sig: Option<String>,
    pub exp: Option<String>,
    pub user: Option<String>,
    pub url: Option<String>,
}

/// Extractor for signed URL authentication.
/// Validates signature from query params (sig, exp, user, url).
pub struct SignedUrlAuthentication(pub String, pub Services);

impl<S> FromRequestParts<S> for SignedUrlAuthentication
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(services): Extension<Services> = Extension::from_request_parts(parts, state)
            .await
            .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

        let Query(query): Query<SignedUrlQuery> = Query::from_request_parts(parts, state)
            .await
            .map_err(|_| Error::Unauthorized)?;

        let (sig, exp_str, user) = match (query.sig, query.exp, query.user) {
            (Some(s), Some(e), Some(u)) => (s, e, u),
            _ => {
                error!("missing required signed URL parameters (sig, exp, user)");
                return Err(Error::Unauthorized);
            }
        };

        let expiry = exp_str.parse::<i64>().map_err(|_| {
            error!("invalid expiry timestamp");
            Error::Unauthorized
        })?;

        let uri = &parts.uri;
        let path = uri.path();

        debug!("Incoming request path: {}", path);
        debug!("Full URI: {}", uri);

        // extract url parameter from the RAW query string (before URL decoding)
        // needed because the signature was generated with the raw b64 string
        let url_param = uri
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

        let url_for_verification = url_param.to_string();

        if !services
            .signature_util
            .verify_signature(&user, expiry, &url_for_verification, &sig)
        {
            error!(
                "Signature was invalid, url: {}, user: {}, expiry: {}, sig: {}",
                url_for_verification, user, expiry, sig
            );
            return Err(Error::Unauthorized);
        }

        Ok(SignedUrlAuthentication(user, services))
    }
}
