use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing::info;

use crate::database::session::DynSessionsRepository;
use crate::database::user::DynUsersRepository;
use crate::server::dtos::session_dto::{NewSessionDto, SessionResponseDto};
use crate::server::dtos::user_dto::ResponseUserDto;
use crate::server::error::{AppResult, Error};
use crate::server::utils::jwt_utils::DynJwtUtil;

pub type DynSessionsService = Arc<dyn SessionsServiceTrait + Send + Sync>;

#[automock]
#[async_trait]
pub trait SessionsServiceTrait {
    async fn new_session(&self, request: NewSessionDto) -> AppResult<SessionResponseDto>;
    async fn refresh_access_token(&self, id: String) -> AppResult<ResponseUserDto>;
}

#[derive(Clone)]
pub struct SessionsService {
    repository: DynSessionsRepository,
    user_repository: DynUsersRepository,
    jwt_util: DynJwtUtil,
}

impl SessionsService {
    pub fn new(
        repository: DynSessionsRepository,
        user_repository: DynUsersRepository,
        jwt_util: DynJwtUtil,
    ) -> Self {
        Self {
            repository,
            user_repository,
            jwt_util,
        }
    }
}

#[async_trait]
impl SessionsServiceTrait for SessionsService {
    async fn new_session(&self, request: NewSessionDto) -> AppResult<SessionResponseDto> {
        let user_id = request.user_id.ok_or_else(|| {
            Error::InternalServerErrorWithContext(
                "User ID missing from validated request".to_string(),
            )
        })?;
        let user_agent = request.user_agent.ok_or_else(|| {
            Error::InternalServerErrorWithContext(
                "User agent missing from validated request, not assigning session for security."
                    .to_string(),
            )
        })?;

        // early fetch of the user to ensure we have the email for jwt token generation
        let user_struct_future = self.user_repository.get_user_by_id(&user_id);

        // set to expire from now + 28 days, this might be mismatched somewhere though
        //
        // this should be kept the same as the access token expiry in @jwt_utils
        let from_now = Duration::from_secs(60 * 60 * 24 * 28);
        let expired_future_time = SystemTime::now().checked_add(from_now).ok_or_else(|| {
            Error::InternalServerErrorWithContext(
                "Session expiry time overflow somehow?".to_string(),
            )
        })?;
        let exp = OffsetDateTime::from(expired_future_time);

        let created_session = self
            .repository
            .new_session(&user_id, user_agent.as_str(), &exp)
            .await?;

        let user_session = user_struct_future.await?;

        info!("session ok, generating jwt token");
        let access_token = self
            .jwt_util
            .new_access_token(user_session.id, &user_session.email)?;

        let refresh_token = self.jwt_util.new_refresh_token(created_session.id)?;

        Ok(SessionResponseDto {
            access_token,
            refresh_token,
        })
    }

    async fn refresh_access_token(&self, id: String) -> AppResult<ResponseUserDto> {
        let user_in_session = self.repository.get_user_by_session_id(&id).await?;

        if let Some(user) = user_in_session {
            info!("session found, making new token...");
            let access_token = self
                .jwt_util
                .new_access_token(user.id.clone(), &user.email)?;

            return Ok(user.into_dto(access_token));
        }

        Err(crate::server::error::Error::Unauthorized)
    }
}
