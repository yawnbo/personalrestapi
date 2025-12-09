use std::sync::Arc;

use tracing::info;

use crate::{
    config::AppConfig,
    database::{Database, RedisDatabase},
    server::{
        services::{
            movie_services::MovieService, ppvsu_services::PpvsuService,
            session_services::SessionsService, stream_services::StreamsService,
            user_services::UsersService,
        },
        utils::{
            argon_utils::{ArgonSecurityUtil, DynArgonUtil},
            jwt_utils::JwtTokenUtil,
            signature_utils::SignatureUtil,
        },
    },
};

use self::{
    movie_services::DynMovieService, ppvsu_services::DynPpvsuService,
    session_services::DynSessionsService, stream_services::DynStreamsService,
    user_services::DynUsersService,
};

use super::utils::jwt_utils::DynJwtUtil;

pub mod movie_services;
pub mod ppvsu_services;
pub mod seed_services;
pub mod session_services;
pub mod stream_services;
pub mod user_services;

// list of services that we are using
#[derive(Clone)]
pub struct Services {
    pub jwt_util: DynJwtUtil,
    pub signature_util: Arc<SignatureUtil>,
    pub users: DynUsersService,
    pub sessions: DynSessionsService,
    pub streams: DynStreamsService,
    pub ppvsu: DynPpvsuService,
    pub movies: DynMovieService,
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

        let ppvsu = Arc::new(PpvsuService::new(redis_repository.clone())) as DynPpvsuService;
        let streams =
            Arc::new(StreamsService::new(redis_repository.clone(), ppvsu.clone())) as DynStreamsService;

        let movies = Arc::new(MovieService::new()) as DynMovieService;

        Self {
            jwt_util,
            signature_util,
            users,
            sessions,
            streams,
            ppvsu,
            movies,
            database: repository,
            redis: redis_repository,
            config,
        }
    }
}
