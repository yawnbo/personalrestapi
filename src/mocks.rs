use crate::database::oidc_identity::MockOidcIdentityRepository;
use crate::database::user::MockUsersRepository;
use crate::server::services::session_services::MockSessionsServiceTrait;
use crate::server::utils::argon_utils::MockArgonUtil;
use crate::server::utils::jwt_utils::MockJwtUtil;
use crate::server::utils::oidc::MockOidcClient;

// mocks for core services
pub struct UsersServiceTestFixture {
    pub mock_repository: MockUsersRepository,
    pub mock_jwt_util: MockJwtUtil,
    pub mock_argon_util: MockArgonUtil,
    pub mock_sessions_services: MockSessionsServiceTrait,
}

impl Default for UsersServiceTestFixture {
    fn default() -> Self {
        UsersServiceTestFixture::new()
    }
}

impl UsersServiceTestFixture {
    pub fn new() -> Self {
        Self {
            mock_repository: MockUsersRepository::new(),
            mock_jwt_util: MockJwtUtil::new(),
            mock_argon_util: MockArgonUtil::new(),
            mock_sessions_services: MockSessionsServiceTrait::new(),
        }
    }
}

// mocks for OIDC service tests
pub struct OidcServiceTestFixture {
    pub mock_oidc_client: MockOidcClient,
    pub mock_user_repository: MockUsersRepository,
    pub mock_oidc_identity_repository: MockOidcIdentityRepository,
    pub mock_sessions_service: MockSessionsServiceTrait,
}

impl Default for OidcServiceTestFixture {
    fn default() -> Self {
        OidcServiceTestFixture::new()
    }
}

impl OidcServiceTestFixture {
    pub fn new() -> Self {
        Self {
            mock_oidc_client: MockOidcClient::new(),
            mock_user_repository: MockUsersRepository::new(),
            mock_oidc_identity_repository: MockOidcIdentityRepository::new(),
            mock_sessions_service: MockSessionsServiceTrait::new(),
        }
    }
}
