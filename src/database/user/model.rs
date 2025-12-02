use std::{sync::Arc, time::SystemTime};

use async_trait::async_trait;
use mockall::automock;
use sqlx::FromRow;
use sqlx::types::time::OffsetDateTime;

#[derive(FromRow, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: String::from("IRFa~VaY2b"),
            email: String::from("stub email"),
            name: String::from("stub name"),
            password: String::from("hashed password"),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
        }
    }
}

pub type DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersRepository {
    async fn create_user(
        &self,
        email: &str,
        name: &str,
        hash_password: &str,
    ) -> anyhow::Result<User>;
    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;
    async fn get_user_by_id(&self, id: &str) -> anyhow::Result<User>;
    async fn update_user(
        &self,
        id: &str,
        email: String,
        name: String,
        password: String,
    ) -> anyhow::Result<User>;
}
