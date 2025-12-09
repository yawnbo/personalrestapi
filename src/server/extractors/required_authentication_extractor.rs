use axum::Extension;
use axum::extract::{FromRequestParts, Query};
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use serde::Deserialize;
use tracing::{debug, error};

use crate::server::error::Error;
use crate::server::services::Services;

// FIXME: PUT THIS IS IN THE DTOS!!!!!!!!!!!
// WHY IS THIS NOT IN THE DTOS WHY DID I PUT THIS HERE??????????
#[derive(Deserialize)]
struct SignedUrlQuery {
    sig: Option<String>,
    exp: Option<String>,
    user: Option<String>,
    token: Option<String>,
}

pub struct RequiredAuthentication(pub String, pub Services);

/// extractor that just gives the user id to the called function or verifies the signed url
impl<S> FromRequestParts<S> for RequiredAuthentication
where
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(services): Extension<Services> = Extension::from_request_parts(parts, state)
            .await
            .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

        // try auth header
        let user_id = if let Some(authorization_header) = parts.headers.get(AUTHORIZATION) {
            let header_value = authorization_header
                .to_str()
                .map_err(|_| Error::Unauthorized)?;

            if !header_value.contains("Bearer") {
                error!("request does not contain valid 'Bearer' prefix for authorization");
                return Err(Error::Unauthorized);
            }

            let tokenized_value: Vec<_> = header_value.split(' ').collect();

            if tokenized_value.len() != 2 || tokenized_value.get(1).is_none() {
                error!("request does not contain a valid token");
                return Err(Error::Unauthorized);
            }

            let token = tokenized_value.into_iter().nth(1).unwrap();
            services
                .jwt_util
                .get_user_id_from_token(token.to_string())
                .map_err(|err| {
                    error!("could not validate user ID from token: {:?}", err);
                    Error::Unauthorized
                })?
        } else {
            // try query params for the signature
            let Query(query): Query<SignedUrlQuery> = Query::from_request_parts(parts, state)
                .await
                .map_err(|_| Error::Unauthorized)?;

            if let (Some(sig), Some(exp_str), Some(user)) = (query.sig, query.exp, query.user) {
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

                if !services.signature_util.verify_signature(
                    &user,
                    expiry,
                    &url_for_verification,
                    &sig,
                ) {
                    error!(
                        "Signature was invalid, url: {}, user: {}, expiry: {}, sig: {}",
                        url_for_verification, user, expiry, sig
                    );
                    return Err(Error::Unauthorized);
                }

                user
            } else if let Some(token) = query.token {
                // i mean like why not i used this like once i think but it shouldn't be called
                services
                    .jwt_util
                    .get_user_id_from_token(token)
                    .map_err(|err| {
                        error!("could not validate user ID from token: {:?}", err);
                        Error::Unauthorized
                    })?
            } else {
                error!("no valid authentication method found");
                return Err(Error::Unauthorized);
            }
        };

        Ok(RequiredAuthentication(user_id, services))
    }
}
