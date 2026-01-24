use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use tracing::{error, info};

use crate::database::oidc_identity::DynOidcIdentityRepository;
use crate::database::user::DynUsersRepository;
use crate::server::dtos::oidc_dto::OidcLoginResponse;
use crate::server::dtos::session_dto::NewSessionDto;
use crate::server::dtos::user_dto::ResponseUserDto;
use crate::server::error::{AppResult, Error};
use crate::server::utils::oidc::{DynOidcClient, OidcAuthUrl};

use super::session_services::DynSessionsService;

pub type DynOidcService = Arc<dyn OidcServiceTrait + Send + Sync>;

#[automock]
#[async_trait]
pub trait OidcServiceTrait {
    fn get_auth_url(&self, provider: &str) -> AppResult<OidcLoginResponse>;
    async fn authenticate(
        &self,
        provider: &str,
        code: &str,
        user_agent: Option<String>,
    ) -> AppResult<(ResponseUserDto, String)>;
    fn get_available_providers(&self) -> Vec<&'static str>;
}

pub struct OidcService {
    providers: HashMap<String, DynOidcClient>,
    user_repository: DynUsersRepository,
    oidc_identity_repository: DynOidcIdentityRepository,
    sessions_service: DynSessionsService,
}

impl OidcService {
    pub fn new(
        providers: HashMap<String, DynOidcClient>,
        user_repository: DynUsersRepository,
        oidc_identity_repository: DynOidcIdentityRepository,
        sessions_service: DynSessionsService,
    ) -> Self {
        Self {
            providers,
            user_repository,
            oidc_identity_repository,
            sessions_service,
        }
    }

    fn get_provider(&self, provider: &str) -> AppResult<&DynOidcClient> {
        self.providers
            .get(provider)
            .ok_or_else(|| Error::BadRequest(format!("unsupported provider: {}", provider)))
    }
}

#[async_trait]
impl OidcServiceTrait for OidcService {
    fn get_auth_url(&self, provider: &str) -> AppResult<OidcLoginResponse> {
        let client = self.get_provider(provider)?;

        // Generate a random state for CSRF protection
        let state = nanoid::nanoid!(32);

        let OidcAuthUrl { url, state } = client
            .generate_auth_url(&state)
            .map_err(|e| Error::InternalServerErrorWithContext(e.to_string()))?;

        Ok(OidcLoginResponse {
            auth_url: url,
            state,
        })
    }

    async fn authenticate(
        &self,
        provider: &str,
        code: &str,
        user_agent: Option<String>,
    ) -> AppResult<(ResponseUserDto, String)> {
        let client = self.get_provider(provider)?;

        // Exchange code for user info
        let user_info = client
            .exchange_code(code)
            .await
            .map_err(|e| {
                error!("failed to exchange code with {}: {:?}", provider, e);
                Error::BadRequest(format!("failed to authenticate with {}", provider))
            })?;

        info!(
            "OIDC user info received - provider: {}, email: {}",
            user_info.provider, user_info.email
        );

        // Check if this OIDC identity already exists
        let existing_identity = self
            .oidc_identity_repository
            .get_identity_by_provider(&user_info.provider, &user_info.provider_user_id)
            .await
            .map_err(|e| Error::InternalServerErrorWithContext(e.to_string()))?;

        let user = if let Some(identity) = existing_identity {
            // User already linked - get existing user
            info!("found existing OIDC identity for user_id: {}", identity.user_id);
            self.user_repository
                .get_user_by_id(&identity.user_id)
                .await
                .map_err(|e| Error::InternalServerErrorWithContext(e.to_string()))?
        } else {
            // Check if user with this email exists
            let existing_user = self
                .user_repository
                .get_user_by_email(&user_info.email)
                .await
                .map_err(|e| Error::InternalServerErrorWithContext(e.to_string()))?;

            if let Some(user) = existing_user {
                // Link this OIDC identity to existing user
                info!(
                    "linking OIDC identity to existing user: {} ({})",
                    user.id, user.email
                );
                self.oidc_identity_repository
                    .create_identity(
                        &user.id,
                        &user_info.provider,
                        &user_info.provider_user_id,
                        Some(user_info.email.clone()),
                    )
                    .await
                    .map_err(|e| Error::InternalServerErrorWithContext(e.to_string()))?;
                user
            } else {
                // Create new OIDC user
                let name = user_info.name.unwrap_or_else(|| {
                    // Use email prefix as fallback name
                    user_info
                        .email
                        .split('@')
                        .next()
                        .unwrap_or("User")
                        .to_string()
                });

                info!(
                    "creating new OIDC user: {} ({})",
                    name, user_info.email
                );

                let new_user = self
                    .user_repository
                    .create_oidc_user(&user_info.email, &name, &user_info.provider)
                    .await
                    .map_err(|e| Error::InternalServerErrorWithContext(e.to_string()))?;

                // Create the OIDC identity record
                self.oidc_identity_repository
                    .create_identity(
                        &new_user.id,
                        &user_info.provider,
                        &user_info.provider_user_id,
                        Some(user_info.email.clone()),
                    )
                    .await
                    .map_err(|e| Error::InternalServerErrorWithContext(e.to_string()))?;

                new_user
            }
        };

        // Create session and return tokens
        let session = self
            .sessions_service
            .new_session(NewSessionDto {
                user_id: Some(user.id.clone()),
                user_agent,
            })
            .await?;

        let user_dto = ResponseUserDto {
            id: user.id,
            name: user.name,
            email: user.email,
            access_token: Some(session.access_token),
        };

        Ok((user_dto, session.refresh_token))
    }

    fn get_available_providers(&self) -> Vec<&'static str> {
        self.providers
            .values()
            .map(|c| c.provider_name())
            .collect()
    }
}
