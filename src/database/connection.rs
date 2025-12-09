use anyhow::{Context, Ok};
use sqlx::postgres::PgPoolOptions;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{PgPool, Pool, Sqlite};
use std::time::Instant;
use tracing::info;

#[derive(Debug, Clone)]
pub enum ConnectionPool {
    Postgres(PgPool),
    Sqlite(Pool<Sqlite>),
}

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: ConnectionPool,
}

impl Database {
    pub async fn connect(connection_string: &str, run_migrations: bool) -> anyhow::Result<Self> {
        let pool = if connection_string.starts_with("postgres://")
            || connection_string.starts_with("postgresql://")
        {
            info!("Connecting to Postgres database");
            let pg_pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(connection_string)
                .await
                .context("Failed to connect to Postgres database")?;

            // these should really be in the migrations instead of the binary but i don't
            // feel like changing this right now :broken_heart: FIXME:
            if run_migrations {
                info!("migrations enabled, running postgres migrations...");
                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS users
                    (
                        id         VARCHAR NOT NULL PRIMARY KEY,
                        name       VARCHAR NOT NULL DEFAULT '',
                        email      VARCHAR NOT NULL DEFAULT '',
                        password   VARCHAR NOT NULL DEFAULT '',
                        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );
                    "#,
                )
                .execute(&pg_pool)
                .await
                .context("Failed to create users table")?;

                sqlx::query("CREATE UNIQUE INDEX IF NOT EXISTS users_email_idx ON users (email);")
                    .execute(&pg_pool)
                    .await
                    .context("Failed to create users email index")?;

                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS sessions
                    (
                        id          VARCHAR PRIMARY KEY,
                        exp         TIMESTAMPTZ NOT NULL,
                        user_id     VARCHAR NOT NULL REFERENCES users (id) ON DELETE CASCADE,
                        user_agent  VARCHAR NOT NULL DEFAULT ''
                    );
                    "#,
                )
                .execute(&pg_pool)
                .await
                .context("Failed to create sessions table")?;

                info!("postgres migrations happy :)");
            }

            ConnectionPool::Postgres(pg_pool)
        } else {
            info!("Connecting to SQLite database");
            let sqlite_pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(connection_string)
                .await
                .context("Failed to connect to SQLite database")?;

            if run_migrations {
                info!("migrations enabled, running sqlite migrations...");
                sqlx::migrate!()
                    .run(&sqlite_pool)
                    .await
                    .context("Failed to run migrations")?;
                info!("sqlite migrations happy :)");
            }

            ConnectionPool::Sqlite(sqlite_pool)
        };

        Ok(Self { pool })
    }

    /// Performs a health check by executing a simple query
    /// Returns response time in milliseconds
    pub async fn health_check(&self) -> anyhow::Result<f64> {
        let start = Instant::now();

        match &self.pool {
            ConnectionPool::Postgres(pool) => {
                // Simple SELECT 1 query for Postgres
                sqlx::query("SELECT 1")
                    .fetch_one(pool)
                    .await
                    .context("PostgreSQL health check failed")?;
            }
            ConnectionPool::Sqlite(pool) => {
                // Simple SELECT 1 query for SQLite
                sqlx::query("SELECT 1")
                    .fetch_one(pool)
                    .await
                    .context("SQLite health check failed")?;
            }
        }

        let elapsed = start.elapsed();
        Ok(elapsed.as_secs_f64() * 1000.0) // Convert to milliseconds
    }

    /// Gets current pool statistics
    /// Returns (active_connections, max_connections)
    pub fn pool_stats(&self) -> (u32, u32) {
        match &self.pool {
            ConnectionPool::Postgres(pool) => {
                let pool_idle = pool.num_idle() as u32;
                let pool_max = pool.options().get_max_connections();
                let pool_active = pool_max - pool_idle;
                (pool_active, pool_max)
            }
            ConnectionPool::Sqlite(pool) => {
                let pool_idle = pool.num_idle() as u32;
                let pool_max = pool.options().get_max_connections();
                let pool_active = pool_max - pool_idle;
                (pool_active, pool_max)
            }
        }
    }
}
