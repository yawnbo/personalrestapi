use crate::database::user::MockUsersRepository;
use crate::server::services::session_services::MockSessionsServiceTrait;
use crate::server::utils::argon_utils::MockArgonUtil;
use crate::server::utils::jwt_utils::MockJwtUtil;

// mocks for core services
//
// note that the ppvsu, seed, movie, and stream services are not mocked and aren't core enough to
// need to be mocked for most tests. Independent tests exist for these services.
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
