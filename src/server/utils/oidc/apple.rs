use anyhow::Context;
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

use super::{OidcAuthUrl, OidcClient, OidcUserInfo};

const APPLE_AUTH_URL: &str = "https://appleid.apple.com/auth/authorize";
const APPLE_TOKEN_URL: &str = "https://appleid.apple.com/auth/token";

#[derive(Deserialize)]
struct AppleTokenResponse {
    access_token: String,
    #[allow(dead_code)]
    token_type: String,
    #[allow(dead_code)]
    expires_in: u64,
    id_token: String,
}

#[derive(Deserialize)]
struct AppleIdTokenClaims {
    sub: String,
    email: Option<String>,
    #[allow(dead_code)]
    email_verified: Option<String>,
}

pub struct AppleOidcClient {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    http_client: Client,
}

impl AppleOidcClient {
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
            http_client: Client::new(),
        }
    }

    /// Decode the ID token to extract claims (without signature verification for simplicity)
    /// In production, you should verify the signature using Apple's public keys
    fn decode_id_token(&self, id_token: &str) -> anyhow::Result<AppleIdTokenClaims> {
        let parts: Vec<&str> = id_token.split('.').collect();
        if parts.len() != 3 {
            anyhow::bail!("invalid ID token format");
        }

        let payload = parts[1];
        // Add padding if needed for base64 decoding
        let padding = 4 - (payload.len() % 4);
        let padded = if padding < 4 {
            format!("{}{}", payload, "=".repeat(padding))
        } else {
            payload.to_string()
        };

        let decoded = base64::Engine::decode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD,
            payload,
        )
        .or_else(|_| {
            base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE, &padded)
        })
        .context("failed to decode ID token payload")?;

        serde_json::from_slice(&decoded).context("failed to parse ID token claims")
    }
}

#[async_trait]
impl OidcClient for AppleOidcClient {
    fn provider_name(&self) -> &'static str {
        "apple"
    }

    fn generate_auth_url(&self, state: &str) -> anyhow::Result<OidcAuthUrl> {
        let url = format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope=email%20name&state={}&response_mode=form_post",
            APPLE_AUTH_URL,
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
        // Exchange code for tokens
        let token_response = self
            .http_client
            .post(APPLE_TOKEN_URL)
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("code", code),
                ("grant_type", "authorization_code"),
                ("redirect_uri", self.redirect_uri.as_str()),
            ])
            .send()
            .await
            .context("failed to exchange code with Apple")?
            .json::<AppleTokenResponse>()
            .await
            .context("failed to parse Apple token response")?;

        // Decode the ID token to get user info
        let claims = self.decode_id_token(&token_response.id_token)?;

        let email = claims
            .email
            .ok_or_else(|| anyhow::anyhow!("no email in Apple ID token"))?;

        Ok(OidcUserInfo {
            provider: self.provider_name().to_string(),
            provider_user_id: claims.sub,
            email,
            name: None, // Apple only provides name on first sign-in via form_post
        })
    }
}
