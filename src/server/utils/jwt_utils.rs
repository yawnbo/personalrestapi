// just random jwt stuff that gets used
use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use mockall::automock;
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;

use crate::config::AppConfig;
use crate::server::error::{AppResult, Error};

// handle jwt stuff
pub type DynJwtUtil = Arc<dyn JwtUtil + Send + Sync>;

#[automock]
pub trait JwtUtil {
    fn new_access_token(&self, user_id: String, email: &str) -> AppResult<String>;
    fn new_refresh_token(&self, sub: String) -> AppResult<String>;
    fn get_user_id_from_token(&self, token: String) -> AppResult<String>;
    fn get_session_id_from_token(&self, token: String) -> AppResult<String>;
}

#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenClaims {
    sub: String,
    user_id: String,
    exp: usize,
    iat: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct RefreshTokenClaims {
    sub: String,
    exp: usize,
    iat: usize,
}

pub struct JwtTokenUtil {
    config: Arc<AppConfig>,
}

impl JwtTokenUtil {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }
}

impl JwtUtil for JwtTokenUtil {
    fn new_access_token(&self, user_id: String, email: &str) -> AppResult<String> {
        let from_now = Duration::from_secs(self.config.access_token_expiry_secs);
        let expired_future_time = SystemTime::now().add(from_now);

        let exp = OffsetDateTime::from(expired_future_time);
        let now = OffsetDateTime::now_utc();

        let claims = AccessTokenClaims {
            sub: String::from(email),
            exp: exp.unix_timestamp() as usize,
            iat: now.unix_timestamp() as usize,
            user_id,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.access_token_secret.as_bytes()),
        )
        .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

        Ok(token)
    }

    fn new_refresh_token(&self, sub: String) -> AppResult<String> {
        let from_now = Duration::from_secs(self.config.refresh_token_expiry_secs);
        let expired_future_time = SystemTime::now().add(from_now);

        let exp = OffsetDateTime::from(expired_future_time);
        let now = OffsetDateTime::now_utc();

        let claims = RefreshTokenClaims {
            sub,
            exp: exp.unix_timestamp() as usize,
            iat: now.unix_timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.refresh_token_secret.as_bytes()),
        )
        .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

        Ok(token)
    }

    fn get_user_id_from_token(&self, token: String) -> AppResult<String> {
        let decoded_token = decode::<AccessTokenClaims>(
            token.as_str(),
            &DecodingKey::from_secret(self.config.access_token_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

        Ok(decoded_token.claims.user_id)
    }

    fn get_session_id_from_token(&self, token: String) -> AppResult<String> {
        let decoded_token = decode::<RefreshTokenClaims>(
            token.as_str(),
            &DecodingKey::from_secret(self.config.refresh_token_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

        Ok(decoded_token.claims.sub)
    }
}
