use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;
use mockall::automock;
use sqlx::FromRow;
use sqlx::types::time::OffsetDateTime;

#[derive(FromRow, Debug)]
pub struct OidcIdentity {
    pub id: String,
    pub user_id: String,
    pub provider: String,
    pub provider_user_id: String,
    pub email: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Default for OidcIdentity {
    fn default() -> Self {
        OidcIdentity {
            id: String::from("oidc-identity-id"),
            user_id: String::from("IRFa~VaY2b"),
            provider: String::from("google"),
            provider_user_id: String::from("google-123"),
            email: Some(String::from("test@example.com")),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
        }
    }
}

pub type DynOidcIdentityRepository = Arc<dyn OidcIdentityRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait OidcIdentityRepository {
    async fn create_identity(
        &self,
        user_id: &str,
        provider: &str,
        provider_user_id: &str,
        email: Option<String>,
    ) -> anyhow::Result<OidcIdentity>;

    async fn get_identity_by_provider(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> anyhow::Result<Option<OidcIdentity>>;

    async fn get_identities_by_user(&self, user_id: &str) -> anyhow::Result<Vec<OidcIdentity>>;
}
