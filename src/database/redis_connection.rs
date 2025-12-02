use anyhow::Context;
use redis::Client;
use redis::aio::MultiplexedConnection;
use tracing::info;

#[derive(Debug, Clone)]
pub struct RedisDatabase {
    pub connection: MultiplexedConnection,
}

// this one is so much simpler god
// not sure if its my problem or upstash but fetching takes a fucking year
impl RedisDatabase {
    pub async fn connect(connection_string: &str) -> anyhow::Result<Self> {
        let client = Client::open(connection_string).context("Failed to create Redis client")?;

        let connection = client
            .get_multiplexed_tokio_connection()
            .await
            .context("Failed to connect to Redis database")?;

        info!("Redis connection established");

        Ok(Self { connection })
    }
}
