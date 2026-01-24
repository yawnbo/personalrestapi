#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum CargoEnv {
    Development,
    Production,
}

#[derive(clap::Parser)]
pub struct AppConfig {
    // production or development
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

    // Token expiration in seconds (defaults: access=30 days, refresh=112 days)
    #[clap(long, env, default_value = "2592000")]
    pub access_token_expiry_secs: u64,

    #[clap(long, env, default_value = "9676800")]
    pub refresh_token_expiry_secs: u64,

    // this should be either * for allowing everything, or a comma seperated list of domains like
    // example.com,something.com
    #[clap(long, env)]
    pub cors_origin: String,

    // same as above but used for preview environments to stress or test the api.
    #[clap(long, env)]
    pub preview_cors_origin: String,

    // optional sentry integration
    #[clap(long, env)]
    pub sentry_dsn: Option<String>,

    // OIDC Provider: Google
    #[clap(long, env)]
    pub oidc_google_client_id: Option<String>,
    #[clap(long, env)]
    pub oidc_google_client_secret: Option<String>,
    #[clap(long, env)]
    pub oidc_google_redirect_uri: Option<String>,

    // OIDC Provider: GitHub
    #[clap(long, env)]
    pub oidc_github_client_id: Option<String>,
    #[clap(long, env)]
    pub oidc_github_client_secret: Option<String>,
    #[clap(long, env)]
    pub oidc_github_redirect_uri: Option<String>,

    // OIDC Provider: Apple
    #[clap(long, env)]
    pub oidc_apple_client_id: Option<String>,
    #[clap(long, env)]
    pub oidc_apple_client_secret: Option<String>,
    #[clap(long, env)]
    pub oidc_apple_redirect_uri: Option<String>,
}
