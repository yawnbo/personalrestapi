#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum CargoEnv {
    Development,
    Production,
}

#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env, value_enum)]
    pub cargo_env: CargoEnv,

    #[clap(long, env, default_value = "5000")]
    pub port: u16,

    // set the default to sqlite as a fallback if someone doesn't put a var but postgres should be used at all times
    #[clap(long, env, default_value = "sqlite:///app/db.sqlite")]
    pub database_url: String,

    #[clap(long, env)]
    pub redis_url: String,

    #[clap(long, env)]
    pub run_migrations: bool,

    #[clap(long, env)]
    pub access_token_secret: String,

    #[clap(long, env)]
    pub refresh_token_secret: String,

    #[clap(long, env)]
    pub cors_origin: String,

    #[clap(long, env)]
    pub preview_cors_origin: String,

    #[clap(long, env)]
    pub seed: bool,

    #[clap(long, env)]
    pub sentry_dsn: Option<String>,
}
