/// actually implement functions in the repository file instead of the model.
use anyhow::Context;
use async_trait::async_trait;
use nanoid::nanoid;
use sqlx::query_as;
use sqlx::types::time::OffsetDateTime;

use crate::database::user::User;
use crate::database::{ConnectionPool, Database};

use super::{Session, SessionsRepository};

#[async_trait]
impl SessionsRepository for Database {
    async fn new_session(
        &self,
        user_id: &str,
        user_agent: &str,
        exp: &OffsetDateTime,
    ) -> anyhow::Result<Session> {
        let session_id = nanoid!();

        match &self.pool {
            ConnectionPool::Postgres(pool) => {
                sqlx::query(
                    r#"
                insert into sessions (id, user_id, user_agent, exp)
                values ($1, $2, $3, $4)
                    "#,
                )
                .bind(&session_id)
                .bind(user_id)
                .bind(user_agent)
                .bind(exp)
                .execute(pool)
                .await
                .context("an unexpected error occured while creating a session")?;
            }
            // this should ideally not really be hit for anything, it could be changed to be used
            // if we panic though
            ConnectionPool::Sqlite(pool) => {
                sqlx::query(
                    r#"
                insert into sessions (id, user_id, user_agent, exp)
                values (?, ?, ?, ?)
                    "#,
                )
                .bind(&session_id)
                .bind(user_id)
                .bind(user_agent)
                .bind(exp)
                .execute(pool)
                .await
                .context("an unexpected error occured while creating a session")?;
            }
        }

        Ok(Session {
            id: session_id,
            user_id: user_id.to_string(),
            user_agent: user_agent.to_string(),
            exp: *exp,
        })
    }

    async fn get_user_by_session_id(&self, id: &str) -> anyhow::Result<Option<User>> {
        match &self.pool {
            ConnectionPool::Postgres(pool) => query_as::<_, User>(
                r#"
                select users.* from users
                inner join sessions
                on users.id = sessions.user_id
                where sessions.exp >= current_timestamp and sessions.id = $1
                    "#,
            )
            .bind(id)
            .fetch_optional(pool)
            .await
            .context("user was not found"),
            ConnectionPool::Sqlite(pool) => query_as::<_, User>(
                r#"
                select users.* from users
                inner join sessions
                on users.id = sessions.user_id
                where sessions.exp >= current_timestamp and sessions.id = ?
                    "#,
            )
            .bind(id)
            .fetch_optional(pool)
            .await
            .context("user was not found"),
        }
    }
}
