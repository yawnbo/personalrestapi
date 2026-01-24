use serde::{Deserialize, Serialize};
use validator::Validate;

/// Response when initiating an OIDC login flow
#[derive(Serialize, Deserialize)]
pub struct OidcLoginResponse {
    pub auth_url: String,
    pub state: String,
}

/// Query parameters received on OIDC callback
#[derive(Deserialize, Validate)]
pub struct OidcCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}
