use axum::extract::Json;
use axum::routing::{get, post, put};
use axum::{Extension, Router};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use tracing::info;

use crate::extractors::{SessionExtractor, UserAgentExtractor};
use crate::server::dtos::user_dto::{
    SignInUserDto, SignUpUserDto, UpdateUserDto, UserAuthenicationResponse,
};
use crate::server::error::AppResult;
use crate::server::extractors::{RequiredAuthentication, ValidationExtractor};
use crate::server::services::Services;

pub struct UserController;
/*
*    api/v1/users/signup - POST - takes in json with email: string, password: string, name: string,
*    creates a new user and returns json with a user object containing the email, name, and a blank
*    access token field, use the signin endpoint for the token.
*
*    api/v1/users/signin - POST - takes in json with email: string, password: string, returns a user
*    object with name, email, and access token bound to the user and useragent, includes a set-cookie
*    header containing the refresh token for the user which should be used for refreshing the login
*
*    api/v1/users/signout - POST - takes in a cookie field with the refresh_token and removes it from
*    the pool
*
*    api/v1/users/whoami - GET - takes in a Authorization header with a Bearer token, (access token) and returns same json as signin.
*    Contains user email, name, and nanoid
*
*    api/v1/users/refresh - GET - takes in a cookie with the refresh_token and returns a new access token
* */
impl UserController {
    pub fn app() -> Router {
        Router::new()
            // i comment out the signups because i don't have proper email sending or verification
            // yet and i don't want my database being slammed
            //
            //.route("/signup", post(Self::signup_user_endpoint))
            .route("/signin", post(Self::signin_user_endpoint))
            .route("/signout", post(Self::signout_user_endpoint))
            .route("/whoami", get(Self::get_current_user_endpoint))
            .route("/refresh", get(Self::refresh_user_endpoint))
            .route("/", put(Self::update_user_endpoint))
    }

    pub async fn signup_user_endpoint(
        Extension(services): Extension<Services>,
        ValidationExtractor(request): ValidationExtractor<SignUpUserDto>,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!(
            "recieved request to create user {:?}/{:?}",
            request.email.as_ref().unwrap_or(&"<missing>".to_string()),
            request.name.as_ref().unwrap_or(&"<missing>".to_string())
        );

        let created_user = services.users.signup_user(request).await?;

        Ok(Json(UserAuthenicationResponse { user: created_user }))
    }
    pub async fn signin_user_endpoint(
        jar: CookieJar,
        Extension(services): Extension<Services>,
        UserAgentExtractor(user_agent): UserAgentExtractor,
        ValidationExtractor(request): ValidationExtractor<SignInUserDto>,
    ) -> AppResult<(CookieJar, Json<UserAuthenicationResponse>)> {
        info!(
            "recieved request to login user {:?}",
            request.email.as_ref().unwrap_or(&"<missing>".to_string())
        );

        let (user, refresh_token) = services.users.signin_user(request, user_agent).await?;

        let cookie = jar.add(Cookie::new("refresh_token", refresh_token.to_string()));

        Ok((cookie, Json(UserAuthenicationResponse { user })))
    }

    pub async fn get_current_user_endpoint(
        RequiredAuthentication(user_id, services): RequiredAuthentication,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to retrieve current user");

        let current_user = services.users.get_current_user(user_id).await?;

        Ok(Json(UserAuthenicationResponse { user: current_user }))
    }

    pub async fn update_user_endpoint(
        RequiredAuthentication(user_id, services): RequiredAuthentication,
        Json(request): Json<UpdateUserDto>,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to update user {:?}", user_id);

        let updated_user = services.users.updated_user(user_id, request).await?;

        Ok(Json(UserAuthenicationResponse { user: updated_user }))
    }

    pub async fn refresh_user_endpoint(
        jar: CookieJar,
        Extension(services): Extension<Services>,
        SessionExtractor(session_id, refresh_token): SessionExtractor,
    ) -> AppResult<(CookieJar, Json<UserAuthenicationResponse>)> {
        info!("recieved request to refresh access token {:?}", session_id);

        let user = services.sessions.refresh_access_token(session_id).await?;

        let cookie = jar.add(Cookie::new("refresh_token", refresh_token));

        Ok((cookie, Json(UserAuthenicationResponse { user })))
    }

    pub async fn signout_user_endpoint(
        jar: CookieJar,
        Extension(services): Extension<Services>,
        SessionExtractor(session_id, _refresh_token): SessionExtractor,
    ) -> AppResult<CookieJar> {
        info!("recieved request to signout session {:?}", session_id);

        services.sessions.refresh_access_token(session_id).await?;

        let cookie = jar.remove(Cookie::from("refresh_token"));

        Ok(cookie)
    }
}
