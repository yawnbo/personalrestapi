mod apple;
mod github;
mod google;

pub use apple::AppleOidcClient;
pub use github::GithubOidcClient;
pub use google::GoogleOidcClient;

use async_trait::async_trait;
use mockall::automock;
use std::sync::Arc;

/// Information about a user obtained from an OIDC provider
#[derive(Debug, Clone)]
pub struct OidcUserInfo {
    pub provider: String,
    pub provider_user_id: String,
    pub email: String,
    pub name: Option<String>,
}

/// Response from initiating an OIDC login flow
#[derive(Debug, Clone)]
pub struct OidcAuthUrl {
    pub url: String,
    pub state: String,
}

pub type DynOidcClient = Arc<dyn OidcClient + Send + Sync>;

/// Trait for OIDC/OAuth providers
#[automock]
#[async_trait]
pub trait OidcClient: Send + Sync {
    /// Returns the provider name (e.g., "google", "github", "apple")
    fn provider_name(&self) -> &'static str;

    /// Generates the authorization URL for the OAuth flow
    fn generate_auth_url(&self, state: &str) -> anyhow::Result<OidcAuthUrl>;

    /// Exchanges the authorization code for user information
    async fn exchange_code(&self, code: &str) -> anyhow::Result<OidcUserInfo>;
}
