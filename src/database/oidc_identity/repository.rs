use anyhow::Context;
use async_trait::async_trait;
use nanoid::nanoid;
use sqlx::query_as;

use crate::database::{ConnectionPool, Database};

use super::model::{OidcIdentity, OidcIdentityRepository};

#[async_trait]
impl OidcIdentityRepository for Database {
    async fn create_identity(
        &self,
        user_id: &str,
        provider: &str,
        provider_user_id: &str,
        email: Option<String>,
    ) -> anyhow::Result<OidcIdentity> {
        let identity_id: String = nanoid!();

        match &self.pool {
            ConnectionPool::Postgres(pool) => {
                sqlx::query(
                    r#"
                insert into oidc_identities (id, user_id, provider, provider_user_id, email)
                values ($1, $2, $3, $4, $5)
                    "#,
                )
                .bind(&identity_id)
                .bind(user_id)
                .bind(provider)
                .bind(provider_user_id)
                .bind(&email)
                .execute(pool)
                .await
                .context("an unexpected error occured while creating the OIDC identity")?;
            }
            ConnectionPool::Sqlite(pool) => {
                sqlx::query(
                    r#"
                insert into oidc_identities (id, user_id, provider, provider_user_id, email)
                values (?, ?, ?, ?, ?)
                    "#,
                )
                .bind(&identity_id)
                .bind(user_id)
                .bind(provider)
                .bind(provider_user_id)
                .bind(&email)
                .execute(pool)
                .await
                .context("an unexpected error occured while creating the OIDC identity")?;
            }
        }

        // Fetch and return the created identity
        match &self.pool {
            ConnectionPool::Postgres(pool) => query_as::<_, OidcIdentity>(
                r#"
                select * from oidc_identities where id = $1
                "#,
            )
            .bind(&identity_id)
            .fetch_one(pool)
            .await
            .context("could not fetch created OIDC identity"),
            ConnectionPool::Sqlite(pool) => query_as::<_, OidcIdentity>(
                r#"
                select * from oidc_identities where id = ?
                "#,
            )
            .bind(&identity_id)
            .fetch_one(pool)
            .await
            .context("could not fetch created OIDC identity"),
        }
    }

    async fn get_identity_by_provider(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> anyhow::Result<Option<OidcIdentity>> {
        match &self.pool {
            ConnectionPool::Postgres(pool) => query_as::<_, OidcIdentity>(
                r#"
                select * from oidc_identities
                where provider = $1 and provider_user_id = $2
                "#,
            )
            .bind(provider)
            .bind(provider_user_id)
            .fetch_optional(pool)
            .await
            .context("unexpected error while querying for OIDC identity"),
            ConnectionPool::Sqlite(pool) => query_as::<_, OidcIdentity>(
                r#"
                select * from oidc_identities
                where provider = ? and provider_user_id = ?
                "#,
            )
            .bind(provider)
            .bind(provider_user_id)
            .fetch_optional(pool)
            .await
            .context("unexpected error while querying for OIDC identity"),
        }
    }

    async fn get_identities_by_user(&self, user_id: &str) -> anyhow::Result<Vec<OidcIdentity>> {
        match &self.pool {
            ConnectionPool::Postgres(pool) => query_as::<_, OidcIdentity>(
                r#"
                select * from oidc_identities where user_id = $1
                "#,
            )
            .bind(user_id)
            .fetch_all(pool)
            .await
            .context("unexpected error while querying for user's OIDC identities"),
            ConnectionPool::Sqlite(pool) => query_as::<_, OidcIdentity>(
                r#"
                select * from oidc_identities where user_id = ?
                "#,
            )
            .bind(user_id)
            .fetch_all(pool)
            .await
            .context("unexpected error while querying for user's OIDC identities"),
        }
    }
}
