use std::sync::Arc;

use argon2::Config;
use mockall::automock;

use crate::server::error::{AppResult, Error};

// argon wrapper
pub type DynArgonUtil = Arc<dyn ArgonUtil + Send + Sync>;

#[automock]
pub trait ArgonUtil {
    fn hash_password(&self, raw_password: &str, salt: &[u8]) -> AppResult<String>;

    fn verify_password(&self, stored_password: &str, attempted_password: String)
    -> AppResult<bool>;
}

// to stay consistent with other utils and easy to change if we need the env in here or something
pub struct ArgonSecurityUtil {}

impl ArgonSecurityUtil {
    pub fn new() -> Self {
        Self {}
    }
}

impl ArgonUtil for ArgonSecurityUtil {
    fn hash_password(&self, raw_password: &str, salt: &[u8]) -> AppResult<String> {
        let password_bytes = raw_password.as_bytes();
        let hashed_password =
            argon2::hash_encoded(password_bytes, salt, &Config::default()).unwrap();
        Ok(hashed_password)
    }

    fn verify_password(
        &self,
        stored_password: &str,
        attempted_password: String,
    ) -> AppResult<bool> {
        let hashes_match = argon2::verify_encoded(stored_password, attempted_password.as_bytes())
            .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;
        Ok(hashes_match)
    }
}
