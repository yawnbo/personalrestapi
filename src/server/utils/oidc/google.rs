use anyhow::Context;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

use super::{OidcAuthUrl, OidcClient, OidcUserInfo};

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_USERINFO_URL: &str = "https://openidconnect.googleapis.com/v1/userinfo";

#[derive(Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
    #[allow(dead_code)]
    token_type: String,
    #[allow(dead_code)]
    expires_in: u64,
    #[allow(dead_code)]
    id_token: Option<String>,
}

#[derive(Deserialize)]
struct GoogleUserInfo {
    sub: String,
    email: String,
    name: Option<String>,
    #[allow(dead_code)]
    picture: Option<String>,
}

pub struct GoogleOidcClient {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    http_client: Client,
}

impl GoogleOidcClient {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl OidcClient for GoogleOidcClient {
    fn provider_name(&self) -> &'static str {
        "google"
    }

    fn generate_auth_url(&self, state: &str) -> anyhow::Result<OidcAuthUrl> {
        let url = format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope=openid%20email%20profile&state={}&access_type=offline",
            GOOGLE_AUTH_URL,
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(state)
        );

        Ok(OidcAuthUrl {
            url,
            state: state.to_string(),
        })
    }

    async fn exchange_code(&self, code: &str) -> anyhow::Result<OidcUserInfo> {
        // Exchange code for access token
        let token_response = self
            .http_client
            .post(GOOGLE_TOKEN_URL)
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("code", code),
                ("grant_type", "authorization_code"),
                ("redirect_uri", self.redirect_uri.as_str()),
            ])
            .send()
            .await
            .context("failed to exchange code with Google")?
            .json::<GoogleTokenResponse>()
            .await
            .context("failed to parse Google token response")?;

        // Fetch user info
        let user_info = self
            .http_client
            .get(GOOGLE_USERINFO_URL)
            .bearer_auth(&token_response.access_token)
            .send()
            .await
            .context("failed to fetch Google user info")?
            .json::<GoogleUserInfo>()
            .await
            .context("failed to parse Google user info")?;

        Ok(OidcUserInfo {
            provider: self.provider_name().to_string(),
            provider_user_id: user_info.sub,
            email: user_info.email,
            name: user_info.name,
        })
    }
}
