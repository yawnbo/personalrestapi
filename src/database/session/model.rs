// using this as the model because its the first in the tree
use std::{sync::Arc, time::SystemTime};

use async_trait::async_trait;
use mockall::automock;
use sqlx::{FromRow, types::time::OffsetDateTime};

use crate::database::user::User;

#[derive(FromRow, Debug)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub exp: OffsetDateTime,
    pub user_agent: String,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            id: String::from("8147a9f8-2845-4f92-9e1d-0c0c6c8db79b"),
            user_id: String::from("IRFa~VaY2b"),
            exp: OffsetDateTime::from(SystemTime::now()),
            user_agent: String::from("stub user agent"),
        }
    }
}

// want to wrap all objects in an arc so that cloning is cheap
pub type DynSessionsRepository = Arc<dyn SessionsRepository + Send + Sync>;

// set all signatures in here and change as needed to keep it nice and clean
#[automock]
#[async_trait]
pub trait SessionsRepository {
    async fn new_session(
        &self,
        user_id: &str,
        user_agent: &str,
        exp: &OffsetDateTime,
    ) -> anyhow::Result<Session>;

    async fn get_user_by_session_id(&self, id: &str) -> anyhow::Result<Option<User>>;
}
