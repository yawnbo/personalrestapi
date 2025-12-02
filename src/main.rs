use std::sync::Arc;

use anyhow::Context;
use clap::Parser;
use dotenvy::dotenv;

use tracing::info;

use api::{AppConfig, ApplicationServer, Database, Logger, RedisDatabase};

// main function!
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let config = Arc::new(AppConfig::parse());

    // init logger and sentry, guards are kept alive to flush logs and maintain sentry connection
    let _guards = Logger::init(config.cargo_env, config.sentry_dsn.clone());

    // as a general rule for logging, keep logs that may clog up stdout out of info and keep things
    // that may be useful for debugging at debug
    info!("logger and env prepped, running migrations...");

    // db's, and migrations
    let db = Database::connect(&config.database_url, config.run_migrations)
        .await
        .expect("Database loading failed");

    info!("connection pool ok, connecting to redis...");

    let redis_db = RedisDatabase::connect(&config.redis_url)
        .await
        .expect("where is the redis connection!!");

    info!("redis connection ok, starting server...");

    // serve the routes
    ApplicationServer::serve(config, db, redis_db)
        .await
        .context("i don't feel like serving the api :)")?;

    Ok(())
}
