// bunch of basic sql for user implementation
use anyhow::Context;
use async_trait::async_trait;
use nanoid::nanoid;
use sqlx::query_as;

use crate::database::{ConnectionPool, Database};

use super::{User, UsersRepository};

#[async_trait]
impl UsersRepository for Database {
    async fn create_user(
        &self,
        email: &str,
        name: &str,
        hash_password: &str,
    ) -> anyhow::Result<User> {
        let user_id: String = nanoid!();

        match &self.pool {
            ConnectionPool::Postgres(pool) => {
                sqlx::query(
                    r#"
                insert into users (id, name, email, password)
                values ($1, $2, $3, $4)
                    "#,
                )
                .bind(&user_id)
                .bind(name)
                .bind(email)
                .bind(hash_password)
                .execute(pool)
                .await
                .context("an unexpected error occured while creating the user")?;
            }
            ConnectionPool::Sqlite(pool) => {
                sqlx::query(
                    r#"
                insert into users (id, name, email, password)
                values (?, ?, ?, ?)
                    "#,
                )
                .bind(&user_id)
                .bind(name)
                .bind(email)
                .bind(hash_password)
                .execute(pool)
                .await
                .context("an unexpected error occured while creating the user")?;
            }
        }

        self.get_user_by_id(&user_id).await
    }

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        match &self.pool {
            ConnectionPool::Postgres(pool) => query_as::<_, User>(
                r#"
                select *
                from users
                where email = $1
                    "#,
            )
            .bind(email)
            .fetch_optional(pool)
            .await
            .context("unexpected error while querying for user by email"),
            ConnectionPool::Sqlite(pool) => query_as::<_, User>(
                r#"
                select *
                from users
                where email = ?
                    "#,
            )
            .bind(email)
            .fetch_optional(pool)
            .await
            .context("unexpected error while querying for user by email"),
        }
    }

    async fn get_user_by_id(&self, id: &str) -> anyhow::Result<User> {
        match &self.pool {
            ConnectionPool::Postgres(pool) => query_as::<_, User>(
                r#"
                select *
                from users
                where id = $1
                    "#,
            )
            .bind(id)
            .fetch_one(pool)
            .await
            .context("user was not found"),
            ConnectionPool::Sqlite(pool) => query_as::<_, User>(
                r#"
                select *
                from users
                where id = ?
                    "#,
            )
            .bind(id)
            .fetch_one(pool)
            .await
            .context("user was not found"),
        }
    }
    async fn update_user(
        &self,
        id: &str,
        email: String,
        name: String,
        password: String,
    ) -> anyhow::Result<User> {
        match &self.pool {
            ConnectionPool::Postgres(pool) => {
                sqlx::query(
                    r#"
                update users
                set
                    name = $1,
                    email = $2,
                    password = $3,
                    updated_at = current_timestamp
                where id = $4
                    "#,
                )
                .bind(&name)
                .bind(&email)
                .bind(&password)
                .bind(id)
                .execute(pool)
                .await
                .context("could not update the user")?;
            }
            ConnectionPool::Sqlite(pool) => {
                sqlx::query(
                    r#"
                update users
                set
                    name = ?,
                    email = ?,
                    password = ?,
                    updated_at = current_timestamp
                where id = ?
                    "#,
                )
                .bind(&name)
                .bind(&email)
                .bind(&password)
                .bind(id)
                .execute(pool)
                .await
                .context("could not update the user")?;
            }
        }

        self.get_user_by_id(id).await
    }
}
