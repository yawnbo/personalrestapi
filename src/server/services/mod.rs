use std::collections::HashMap;
use std::sync::Arc;

use tracing::info;

use crate::{
    config::AppConfig,
    database::{Database, RedisDatabase},
    server::{
        services::{
            oidc_services::OidcService, session_services::SessionsService,
            user_services::UsersService,
        },
        utils::{
            argon_utils::{ArgonSecurityUtil, DynArgonUtil},
            jwt_utils::JwtTokenUtil,
            oidc::{AppleOidcClient, DynOidcClient, GithubOidcClient, GoogleOidcClient},
            signature_utils::SignatureUtil,
        },
    },
};

use self::{
    oidc_services::DynOidcService, session_services::DynSessionsService,
    user_services::DynUsersService,
};

use super::utils::jwt_utils::DynJwtUtil;

pub mod oidc_services;
pub mod session_services;
pub mod user_services;

// Core services for authentication framework
#[derive(Clone)]
pub struct Services {
    pub jwt_util: DynJwtUtil,
    pub signature_util: Arc<SignatureUtil>,
    pub users: DynUsersService,
    pub sessions: DynSessionsService,
    pub oidc: Option<DynOidcService>,
    pub database: Arc<Database>,
    pub redis: Arc<RedisDatabase>,
    pub config: Arc<AppConfig>,
}

impl Services {
    pub fn new(db: Database, redis_db: RedisDatabase, config: Arc<AppConfig>) -> Self {
        info!("starting util services...");

        // config can be passed into the security util but it's not currently used as hashes and
        // salts are generated and stored in the db per user
        let security_service = Arc::new(ArgonSecurityUtil::new()) as DynArgonUtil;
        let jwt_util = Arc::new(JwtTokenUtil::new(config.clone())) as DynJwtUtil;
        let signature_util = Arc::new(SignatureUtil::new(config.access_token_secret.clone()));

        info!("jwt and hashing ok, starting remaining services...");
        let repository = Arc::new(db);
        let redis_repository = Arc::new(redis_db);

        // these repos could be bundled together but the first one is just the session
        // implementation while the second is for users
        let sessions = Arc::new(SessionsService::new(
            repository.clone(),
            repository.clone(),
            jwt_util.clone(),
        )) as DynSessionsService;

        let users = Arc::new(UsersService::new(
            repository.clone(),
            security_service,
            jwt_util.clone(),
            sessions.clone(),
        )) as DynUsersService;

        // Build OIDC providers based on config
        let mut oidc_providers: HashMap<String, DynOidcClient> = HashMap::new();

        if let (Some(id), Some(secret), Some(uri)) = (
            config.oidc_google_client_id.as_ref(),
            config.oidc_google_client_secret.as_ref(),
            config.oidc_google_redirect_uri.as_ref(),
        ) {
            info!("configuring Google OIDC provider");
            oidc_providers.insert(
                "google".to_string(),
                Arc::new(GoogleOidcClient::new(
                    id.clone(),
                    secret.clone(),
                    uri.clone(),
                )) as DynOidcClient,
            );
        }

        if let (Some(id), Some(secret), Some(uri)) = (
            config.oidc_github_client_id.as_ref(),
            config.oidc_github_client_secret.as_ref(),
            config.oidc_github_redirect_uri.as_ref(),
        ) {
            info!("configuring GitHub OIDC provider");
            oidc_providers.insert(
                "github".to_string(),
                Arc::new(GithubOidcClient::new(
                    id.clone(),
                    secret.clone(),
                    uri.clone(),
                )) as DynOidcClient,
            );
        }

        if let (Some(id), Some(secret), Some(uri)) = (
            config.oidc_apple_client_id.as_ref(),
            config.oidc_apple_client_secret.as_ref(),
            config.oidc_apple_redirect_uri.as_ref(),
        ) {
            info!("configuring Apple OIDC provider");
            oidc_providers.insert(
                "apple".to_string(),
                Arc::new(AppleOidcClient::new(
                    id.clone(),
                    secret.clone(),
                    uri.clone(),
                )) as DynOidcClient,
            );
        }

        let oidc = if !oidc_providers.is_empty() {
            info!(
                "OIDC service enabled with {} provider(s)",
                oidc_providers.len()
            );
            Some(Arc::new(OidcService::new(
                oidc_providers,
                repository.clone(),
                repository.clone(),
                sessions.clone(),
            )) as DynOidcService)
        } else {
            info!("no OIDC providers configured");
            None
        };

        Self {
            jwt_util,
            signature_util,
            users,
            sessions,
            oidc,
            database: repository,
            redis: redis_repository,
            config,
        }
    }
}
