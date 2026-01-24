use std::collections::HashMap;
use std::sync::Arc;

use api::{
    database::oidc_identity::{DynOidcIdentityRepository, OidcIdentity},
    database::user::{DynUsersRepository, User},
    mocks::OidcServiceTestFixture,
    server::{
        dtos::session_dto::SessionResponseDto,
        services::{
            oidc_services::{OidcService, OidcServiceTrait},
            session_services::DynSessionsService,
        },
        utils::oidc::{DynOidcClient, OidcAuthUrl, OidcUserInfo},
    },
};
use mockall::predicate::*;

#[tokio::test]
async fn return_existing_user_when_oidc_identity_exists() {
    // arrange
    let mut fixture = OidcServiceTestFixture::default();

    let user_info = OidcUserInfo {
        provider: "google".to_string(),
        provider_user_id: "google-123".to_string(),
        email: "test@example.com".to_string(),
        name: Some("Test User".to_string()),
    };

    fixture
        .mock_oidc_client
        .expect_exchange_code()
        .with(eq("auth-code"))
        .times(1)
        .return_once(move |_| Ok(user_info));

    fixture
        .mock_oidc_identity_repository
        .expect_get_identity_by_provider()
        .with(eq("google"), eq("google-123"))
        .times(1)
        .return_once(move |_, _| Ok(Some(OidcIdentity::default())));

    fixture
        .mock_user_repository
        .expect_get_user_by_id()
        .with(eq("IRFa~VaY2b"))
        .times(1)
        .return_once(move |_| Ok(User::default()));

    fixture
        .mock_sessions_service
        .expect_new_session()
        .times(1)
        .return_once(move |_| Ok(SessionResponseDto::default()));

    let mut providers: HashMap<String, DynOidcClient> = HashMap::new();
    providers.insert("google".to_string(), Arc::new(fixture.mock_oidc_client));

    let oidc_service = OidcService::new(
        providers,
        Arc::new(fixture.mock_user_repository) as DynUsersRepository,
        Arc::new(fixture.mock_oidc_identity_repository) as DynOidcIdentityRepository,
        Arc::new(fixture.mock_sessions_service) as DynSessionsService,
    );

    // act
    let result = oidc_service
        .authenticate("google", "auth-code", Some("test-agent".to_string()))
        .await;

    // assert
    assert!(result.is_ok());
    let (user, _refresh_token) = result.unwrap();
    assert_eq!(user.email, "stub email");
}

#[tokio::test]
async fn link_existing_user_when_email_matches() {
    // arrange
    let mut fixture = OidcServiceTestFixture::default();

    let user_info = OidcUserInfo {
        provider: "google".to_string(),
        provider_user_id: "google-456".to_string(),
        email: "existing@example.com".to_string(),
        name: Some("Existing User".to_string()),
    };

    fixture
        .mock_oidc_client
        .expect_exchange_code()
        .with(eq("auth-code"))
        .times(1)
        .return_once(move |_| Ok(user_info));

    // No existing OIDC identity
    fixture
        .mock_oidc_identity_repository
        .expect_get_identity_by_provider()
        .with(eq("google"), eq("google-456"))
        .times(1)
        .return_once(move |_, _| Ok(None));

    // User exists with this email
    fixture
        .mock_user_repository
        .expect_get_user_by_email()
        .with(eq("existing@example.com"))
        .times(1)
        .return_once(move |_| Ok(Some(User::default())));

    // Should create new identity linking to existing user
    fixture
        .mock_oidc_identity_repository
        .expect_create_identity()
        .times(1)
        .return_once(move |_, _, _, _| Ok(OidcIdentity::default()));

    fixture
        .mock_sessions_service
        .expect_new_session()
        .times(1)
        .return_once(move |_| Ok(SessionResponseDto::default()));

    let mut providers: HashMap<String, DynOidcClient> = HashMap::new();
    providers.insert("google".to_string(), Arc::new(fixture.mock_oidc_client));

    let oidc_service = OidcService::new(
        providers,
        Arc::new(fixture.mock_user_repository) as DynUsersRepository,
        Arc::new(fixture.mock_oidc_identity_repository) as DynOidcIdentityRepository,
        Arc::new(fixture.mock_sessions_service) as DynSessionsService,
    );

    // act
    let result = oidc_service
        .authenticate("google", "auth-code", Some("test-agent".to_string()))
        .await;

    // assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn create_new_user_when_oidc_email_not_found() {
    // arrange
    let mut fixture = OidcServiceTestFixture::default();

    let user_info = OidcUserInfo {
        provider: "google".to_string(),
        provider_user_id: "google-789".to_string(),
        email: "new@example.com".to_string(),
        name: Some("New User".to_string()),
    };

    fixture
        .mock_oidc_client
        .expect_exchange_code()
        .with(eq("auth-code"))
        .times(1)
        .return_once(move |_| Ok(user_info));

    // No existing OIDC identity
    fixture
        .mock_oidc_identity_repository
        .expect_get_identity_by_provider()
        .with(eq("google"), eq("google-789"))
        .times(1)
        .return_once(move |_, _| Ok(None));

    // No existing user with this email
    fixture
        .mock_user_repository
        .expect_get_user_by_email()
        .with(eq("new@example.com"))
        .times(1)
        .return_once(move |_| Ok(None));

    // Should create new OIDC user
    fixture
        .mock_user_repository
        .expect_create_oidc_user()
        .with(eq("new@example.com"), eq("New User"), eq("google"))
        .times(1)
        .return_once(move |_, _, _| Ok(User::default()));

    // Should create identity for new user
    fixture
        .mock_oidc_identity_repository
        .expect_create_identity()
        .times(1)
        .return_once(move |_, _, _, _| Ok(OidcIdentity::default()));

    fixture
        .mock_sessions_service
        .expect_new_session()
        .times(1)
        .return_once(move |_| Ok(SessionResponseDto::default()));

    let mut providers: HashMap<String, DynOidcClient> = HashMap::new();
    providers.insert("google".to_string(), Arc::new(fixture.mock_oidc_client));

    let oidc_service = OidcService::new(
        providers,
        Arc::new(fixture.mock_user_repository) as DynUsersRepository,
        Arc::new(fixture.mock_oidc_identity_repository) as DynOidcIdentityRepository,
        Arc::new(fixture.mock_sessions_service) as DynSessionsService,
    );

    // act
    let result = oidc_service
        .authenticate("google", "auth-code", Some("test-agent".to_string()))
        .await;

    // assert
    assert!(result.is_ok());
}

#[test]
fn return_error_for_unsupported_provider() {
    // arrange
    let providers: HashMap<String, DynOidcClient> = HashMap::new();
    let fixture = OidcServiceTestFixture::default();

    let oidc_service = OidcService::new(
        providers,
        Arc::new(fixture.mock_user_repository) as DynUsersRepository,
        Arc::new(fixture.mock_oidc_identity_repository) as DynOidcIdentityRepository,
        Arc::new(fixture.mock_sessions_service) as DynSessionsService,
    );

    // act
    let result = oidc_service.get_auth_url("unsupported");

    // assert
    assert!(result.is_err());
}

#[test]
fn return_auth_url_for_supported_provider() {
    // arrange
    let mut fixture = OidcServiceTestFixture::default();

    fixture
        .mock_oidc_client
        .expect_generate_auth_url()
        .times(1)
        .return_once(move |state| {
            Ok(OidcAuthUrl {
                url: "https://accounts.google.com/auth".to_string(),
                state: state.to_string(),
            })
        });

    let mut providers: HashMap<String, DynOidcClient> = HashMap::new();
    providers.insert("google".to_string(), Arc::new(fixture.mock_oidc_client));

    let oidc_service = OidcService::new(
        providers,
        Arc::new(fixture.mock_user_repository) as DynUsersRepository,
        Arc::new(fixture.mock_oidc_identity_repository) as DynOidcIdentityRepository,
        Arc::new(fixture.mock_sessions_service) as DynSessionsService,
    );

    // act
    let result = oidc_service.get_auth_url("google");

    // assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.auth_url, "https://accounts.google.com/auth");
}
