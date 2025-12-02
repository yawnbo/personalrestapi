use lazy_static::lazy_static;
use tracing::info;

use crate::server::{dtos::user_dto::SignInUserDto, error::AppResult};

use super::{Services, user_services::DynUsersService};

// test data for seeding, this can be done lazily
lazy_static! {
    static ref TEST_USER_1_NAME: &'static str = "yawnbo";
    static ref TEST_USER_1_EMAIL: &'static str = "spam@yawnbo.xyz";
    static ref TEST_USER_1_PASSWORD: &'static str = "e98ji213klla-091msaidjapok1mm0dais";
}

pub struct SeedService {
    user_services: DynUsersService,
}

impl SeedService {
    pub fn new(services: Services) -> Self {
        Self {
            user_services: services.users,
        }
    }

    pub async fn seed(&self) -> AppResult<()> {
        // assume that if we have an active user in the users table, data has been seeded
        let seed_data_exists = self
            .user_services
            .signin_user(
                SignInUserDto {
                    email: Some(String::from(*TEST_USER_1_EMAIL)),
                    password: Some(String::from(*TEST_USER_1_PASSWORD)),
                },
                Some(String::from("Seed Agent")),
            )
            .await
            .is_ok();

        if seed_data_exists {
            info!("data has already been seeded, bypassing test data setup");
            return Ok(());
        }
        info!("seeding users...");

        info!("seed ran successfully!");
        Ok(())
    }
}
