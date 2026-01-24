use anyhow::Context;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

use super::{OidcAuthUrl, OidcClient, OidcUserInfo};

const GITHUB_AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const GITHUB_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const GITHUB_USER_URL: &str = "https://api.github.com/user";
const GITHUB_EMAILS_URL: &str = "https://api.github.com/user/emails";

#[derive(Deserialize)]
struct GithubTokenResponse {
    access_token: String,
    #[allow(dead_code)]
    token_type: String,
    #[allow(dead_code)]
    scope: String,
}

#[derive(Deserialize)]
struct GithubUser {
    id: i64,
    #[allow(dead_code)]
    login: String,
    name: Option<String>,
    email: Option<String>,
}

#[derive(Deserialize)]
struct GithubEmail {
    email: String,
    primary: bool,
    #[allow(dead_code)]
    verified: bool,
}

pub struct GithubOidcClient {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    http_client: Client,
}

impl GithubOidcClient {
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
impl OidcClient for GithubOidcClient {
    fn provider_name(&self) -> &'static str {
        "github"
    }

    fn generate_auth_url(&self, state: &str) -> anyhow::Result<OidcAuthUrl> {
        let url = format!(
            "{}?client_id={}&redirect_uri={}&scope=user:email&state={}",
            GITHUB_AUTH_URL,
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
            .post(GITHUB_TOKEN_URL)
            .header("Accept", "application/json")
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("code", code),
                ("redirect_uri", self.redirect_uri.as_str()),
            ])
            .send()
            .await
            .context("failed to exchange code with GitHub")?
            .json::<GithubTokenResponse>()
            .await
            .context("failed to parse GitHub token response")?;

        // Fetch user info
        let user_info = self
            .http_client
            .get(GITHUB_USER_URL)
            .header("Authorization", format!("Bearer {}", token_response.access_token))
            .header("User-Agent", "rust-auth-framework")
            .send()
            .await
            .context("failed to fetch GitHub user info")?
            .json::<GithubUser>()
            .await
            .context("failed to parse GitHub user info")?;

        // Get primary email if not included in user info
        let email = if let Some(email) = user_info.email {
            email
        } else {
            // Fetch emails separately
            let emails = self
                .http_client
                .get(GITHUB_EMAILS_URL)
                .header("Authorization", format!("Bearer {}", token_response.access_token))
                .header("User-Agent", "rust-auth-framework")
                .send()
                .await
                .context("failed to fetch GitHub emails")?
                .json::<Vec<GithubEmail>>()
                .await
                .context("failed to parse GitHub emails")?;

            emails
                .into_iter()
                .find(|e| e.primary)
                .map(|e| e.email)
                .ok_or_else(|| anyhow::anyhow!("no primary email found for GitHub user"))?
        };

        Ok(OidcUserInfo {
            provider: self.provider_name().to_string(),
            provider_user_id: user_info.id.to_string(),
            email,
            name: user_info.name,
        })
    }
}
