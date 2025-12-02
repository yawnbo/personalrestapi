use axum::Router;
use axum::extract::{Json, Query};
use axum::http::header;
use axum::routing::{get, post};
use base64::{Engine, engine::general_purpose::URL_SAFE};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use validator::Validate;

use crate::server::dtos::movie_dto::{
    CaptionInfo, DecryptMovieRequest, DecryptMovieResponse, EncryptMovieRequest,
    EncryptMovieResponse, GetMovieLinkResponse, VerifyKeyResponse, VidLinkResponse,
};
use crate::server::error::{AppResult, Error};
use crate::server::extractors::RequiredAuthentication;
use crate::server::utils::signature_utils::SignatureUtil;

pub struct MovieController;

#[derive(Debug, Deserialize)]
pub struct GetMovieLinkQuery {
    /// plain movie tv id
    pub id: String,
    /// option for episode number for tv shows
    pub ep: Option<String>,
}

#[derive(Serialize)]
pub struct SignedMovieLinkResponse {
    pub signed_url: String,
    pub expires_at: i64,
    pub captions: Vec<CaptionInfo>,
}

impl MovieController {
    pub fn app() -> Router {
        Router::new()
            .route("/decrypt", post(Self::decrypt_movie_endpoint))
            .route("/encrypt", post(Self::encrypt_movie_endpoint))
            .route("/link", get(Self::get_movie_link_endpoint))
            .route(
                "/link/signed-url",
                get(Self::get_signed_movie_link_endpoint),
            )
            .route("/verify-key", get(Self::verify_key_endpoint))
    }

    /// these were confusing asf so they're like the only ones docs other than users
    /// POST /api/v1/movies/decrypt
    ///
    /// decrypt an encrypted movie ID and extract the timestamp
    /// requires authentication.
    ///
    /// request body:
    /// ```json
    /// {
    ///   "encrypted_id": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAoJqtBAkw0H1UB314p1Og5cGACc99c2cZ2cRK4FF2XA"
    /// }
    /// ```
    ///
    /// response:
    /// ```json
    /// {
    ///   "movie_id": "1311031",
    ///   "timestamp": 1761037963,
    ///   "key_valid": true,
    ///   "timestamp_readable": "2025-10-21T06:12:43+00:00"
    /// }
    /// ```
    pub async fn decrypt_movie_endpoint(
        RequiredAuthentication(_user_id, services): RequiredAuthentication,
        Json(payload): Json<DecryptMovieRequest>,
    ) -> AppResult<Json<DecryptMovieResponse>> {
        info!("received request to decrypt movie ID");

        payload
            .validate()
            .map_err(|e| Error::BadRequest(format!("Validation error: {}", e)))?;

        let result = services
            .movies
            .decrypt_movie_id(&payload.encrypted_id)
            .await?;

        info!(
            "successfully decrypted movie ID: {}, timestamp: {}",
            result.movie_id, result.timestamp
        );

        Ok(Json(result))
    }

    /// POST /api/v1/movies/encrypt
    ///
    /// encrypt a movie ID and embed timestamp
    /// requires auth
    ///
    /// request body:
    /// ```json
    /// {
    ///   "movie_id": "1311031",
    ///   "timestamp": 1761037963  // optional, uses current time if not provided
    /// }
    /// ```
    ///
    /// response:
    /// ```json
    /// {
    ///   "movie_id": "1311031",
    ///   "encrypted_id": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAoJqtBAkw0H1UB314p1Og5cGACc99c2cZ2cRK4FF2XA",
    ///   "timestamp": 1761037963,
    ///   "timestamp_readable": "2025-10-21T06:12:43+00:00"
    /// }
    /// ```
    pub async fn encrypt_movie_endpoint(
        RequiredAuthentication(_user_id, services): RequiredAuthentication,
        Json(payload): Json<EncryptMovieRequest>,
    ) -> AppResult<Json<EncryptMovieResponse>> {
        info!("received request to encrypt movie ID: {}", payload.movie_id);

        payload
            .validate()
            .map_err(|e| Error::BadRequest(format!("Validation error: {}", e)))?;

        let result = services
            .movies
            .encrypt_movie_id(&payload.movie_id, payload.timestamp)
            .await?;

        info!(
            "successfully encrypted movie ID: {}, timestamp: {}",
            result.movie_id, result.timestamp
        );

        Ok(Json(result))
    }

    /// GET /api/v1/movies/link
    ///
    /// get video stream and caption URLs from a certain website that is named elsewhere
    /// requires auth
    ///
    /// FIXME:
    /// same defn as the getmovielinkquery (why did i not put this with the
    /// other dtos??????)
    ///
    /// params:
    /// - `id`
    /// - `ep`
    ///
    /// response:
    /// ```json
    /// {
    ///   "playlist_url": "/api/v1/proxy?url=...&movie=true",
    ///   "captions": [
    ///     {"language": "English", "url": "https://..."}
    ///   ]
    /// }
    /// ```
    pub async fn get_movie_link_endpoint(
        RequiredAuthentication(_user_id, services): RequiredAuthentication,
        Query(params): Query<GetMovieLinkQuery>,
    ) -> AppResult<Json<GetMovieLinkResponse>> {
        info!(
            "received request to get movie link for movie ID: {}",
            params.id
        );

        // encrypt for 2 mins in the future
        // this should be enough time....
        // i could also sync this with the scraped key time time but no need
        let future_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| {
                crate::server::error::Error::InternalServerErrorWithContext(
                    "System time before UNIX epoch".to_string(),
                )
            })?
            .as_secs() as u32
            + 240; // 4 mins

        let encrypt_result = services
            .movies
            .encrypt_movie_id(&params.id, Some(future_timestamp))
            .await?;
        let encrypted_id = encrypt_result.encrypted_id;

        debug!(
            "encrypted movie ID: {} with timestamp: {}",
            encrypted_id, future_timestamp
        );

        let movie_id = encrypted_id;

        let (content_type, api_url) = if let Some(ref ep) = params.ep {
            // uhhhhh no api is being called here there is no movies being scraped
            let url = format!(
                "https://vidlink.pro/api/b/tv/{}/{}?multilang=0",
                movie_id, ep
            );
            ("tv", url)
        } else {
            let url = format!("https://vidlink.pro/api/b/movie/{}?multilang=0", movie_id);
            ("movie", url)
        };

        info!("calling some API: {}", api_url);

        // i don't think it actually has to be this way but whatever i know it has to be
        // there
        let referer = if let Some(ref ep) = params.ep {
            format!("https://vidlink.pro/tv/{}/{}", movie_id, ep)
        } else {
            format!("https://vidlink.pro/movie/{}", movie_id)
        };

        let client = reqwest::Client::new();
        let response = client
            .get(&api_url)
            // 90% of these are really not needed but i also don't want to be randomly ip
            // banned because of some user agent check (or cloudflare you never know)
            .header(header::HOST, "vidlink.pro")
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:145.0) Gecko/20100101 Firefox/145.0",
            )
            .header(header::ACCEPT, "*/*")
            .header(header::ACCEPT_LANGUAGE, "en-US,en;q=0.5")
            .header(header::REFERER, referer)
            .header("Sec-GPC", "1")
            .header(header::CONNECTION, "keep-alive")
            .header("Sec-Fetch-Dest", "empty")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Site", "same-origin")
            .header("Priority", "u=4")
            .send()
            .await
            .map_err(|e| {
                tracing::error!("failed to call vidlink.pro API: {}", e);
                Error::InternalServerErrorWithContext(format!("Failed to fetch stream: {}", e))
            })?;

        if !response.status().is_success() {
            tracing::error!("vidlink.pro API returned error: {}", response.status());
            return Err(Error::NotFound(format!(
                "Stream not found for movie ID: {}",
                movie_id
            )));
        }

        // this can be empty for things that aren't out or released yet so i should
        // probably throw a softer error so it isn't caught by sentry but it should be ok
        let vidlink_response: VidLinkResponse = response.json().await.map_err(|e| {
            info!("Unable to serialize vidlink response: {}", e,);
            Error::NotFound("Unable to obtain suitable response from api".to_string())
        })?;

        debug!(
            "received playlist URL: {}",
            vidlink_response.stream.playlist
        );

        let encoded_playlist_url = URL_SAFE
            .encode(vidlink_response.stream.playlist.as_bytes())
            .trim_end_matches('=')
            .to_string();
        let playlist_url = format!("/api/v1/proxy?url={}&schema=movie", encoded_playlist_url);

        // don't ask about these they're soooooo annoying
        let captions: Vec<CaptionInfo> = vidlink_response
            .stream
            .captions
            .into_iter()
            .map(|c| {
                let encoded_caption_url = URL_SAFE
                    .encode(c.url.as_bytes())
                    .trim_end_matches('=')
                    .to_string();
                let proxied_url = format!("/api/v1/proxy/captions?url={}", encoded_caption_url);

                CaptionInfo {
                    language: c.language,
                    url: proxied_url,
                }
            })
            .collect();

        info!(
            "successfully retrieved stream link for movie ID: {} ({})",
            movie_id, content_type
        );

        Ok(Json(GetMovieLinkResponse {
            playlist_url,
            captions,
        }))
    }

    /// GET /api/v1/movies/verify-key
    ///
    /// verify the encryption key is still valid by testing against known samples (not really
    /// used)
    ///
    /// response:
    /// ```json
    /// {
    ///   "key_valid": true,
    ///   "message": "Encryption key is valid and working correctly",
    ///   "key_hex": "c75136c5668bbfe65a7ecad431a745db68b5f381555b38d8f6c699449cf11fcd"
    /// }
    /// ```
    pub async fn verify_key_endpoint(
        RequiredAuthentication(_user_id, services): RequiredAuthentication,
    ) -> AppResult<Json<VerifyKeyResponse>> {
        info!("received request to verify encryption key");

        let result = services.movies.verify_key().await?;

        info!("key verification result: {}", result.key_valid);

        Ok(Json(result))
    }

    /// GET /api/v1/movies/link/signed-url
    ///
    /// get a signed URL for streaming with Safari/iOS native HLS, this is preffered to the normal
    /// method now as it's more versatile, but the old method is still supported because it's still
    /// secure and works fine (i think)
    ///
    /// returns the playlist URL with HMAC signature auth
    ///
    /// params:
    /// - `id`:
    /// - `ep`:
    ///
    /// response:
    /// ```json
    /// {
    ///   "signed_url": "/api/v1/proxy?url=...&schema=movie&sig=...&exp=...&user=...",
    ///   "expires_at": 1234567890,
    ///   "captions": [{"language": "English", "url": "..."}]
    /// }
    /// ```
    pub async fn get_signed_movie_link_endpoint(
        RequiredAuthentication(user_id, services): RequiredAuthentication,
        Query(params): Query<GetMovieLinkQuery>,
    ) -> AppResult<Json<SignedMovieLinkResponse>> {
        info!(
            "received request to get signed movie link for movie ID: {}",
            params.id
        );

        let movie_link_response = Self::get_movie_link_internal(&services, &params).await?;

        let encoded_url = movie_link_response
            .playlist_url
            .split("url=")
            .nth(1)
            .and_then(|s| s.split('&').next())
            .ok_or_else(|| {
                Error::InternalServerErrorWithContext("Failed to parse playlist URL".to_string())
            })?;

        // sign just the encoded url param, not the full path
        // i got issues with gen and verification before so
        let expiry = SignatureUtil::generate_expiry(12);
        let url_for_signature = encoded_url.to_string();

        debug!("Generating signature for movie:");
        debug!("  URL for signature: {}", url_for_signature);
        debug!("  User: {}", user_id);
        debug!("  Expiry: {}", expiry);

        let signature =
            services
                .signature_util
                .generate_signature(&user_id, expiry, &url_for_signature);

        let signed_url = format!(
            "/api/v1/proxy?url={}&schema=movie&sig={}&exp={}&user={}",
            encoded_url,
            signature,
            expiry,
            urlencoding::encode(&user_id)
        );

        info!(
            "generated signed movie URL for {} (expires: {})",
            params.id, expiry
        );
        debug!("  Full signed URL: {}", signed_url);

        Ok(Json(SignedMovieLinkResponse {
            signed_url,
            expires_at: expiry,
            captions: movie_link_response.captions,
        }))
    }

    // internal helper to get movie link without authentication wrapper
    async fn get_movie_link_internal(
        services: &crate::server::services::Services,
        params: &GetMovieLinkQuery,
    ) -> AppResult<GetMovieLinkResponse> {
        // encrypt the movie ID with timestamp 4 minutes in the future
        // per their key making
        let future_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| {
                crate::server::error::Error::InternalServerErrorWithContext(
                    "System time before UNIX epoch".to_string(),
                )
            })?
            .as_secs() as u32
            + 240;

        let encrypt_result = services
            .movies
            .encrypt_movie_id(&params.id, Some(future_timestamp))
            .await?;
        let encrypted_id = encrypt_result.encrypted_id;

        debug!(
            "encrypted movie ID: {} with timestamp: {}",
            encrypted_id, future_timestamp
        );

        let movie_id = encrypted_id;

        let (content_type, api_url) = if let Some(ref ep) = params.ep {
            let url = format!(
                "https://vidlink.pro/api/b/tv/{}/{}?multilang=0",
                movie_id, ep
            );
            ("tv", url)
        } else {
            let url = format!("https://vidlink.pro/api/b/movie/{}?multilang=0", movie_id);
            ("movie", url)
        };

        info!("calling vidlink.pro API: {}", api_url);

        let referer = if let Some(ref ep) = params.ep {
            format!("https://vidlink.pro/tv/{}/{}", movie_id, ep)
        } else {
            format!("https://vidlink.pro/movie/{}", movie_id)
        };

        let client = reqwest::Client::new();
        let response = client
            .get(&api_url)
            .header(header::HOST, "vidlink.pro")
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:145.0) Gecko/20100101 Firefox/145.0",
            )
            .header(header::ACCEPT, "*/*")
            .header(header::ACCEPT_LANGUAGE, "en-US,en;q=0.5")
            .header(header::REFERER, referer)
            .header("Sec-GPC", "1")
            .header(header::CONNECTION, "keep-alive")
            .header("Sec-Fetch-Dest", "empty")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Site", "same-origin")
            .header("Priority", "u=4")
            .send()
            .await
            .map_err(|e| {
                tracing::error!("failed to call vidlink.pro API: {}", e);
                Error::InternalServerErrorWithContext(format!("Failed to fetch stream: {}", e))
            })?;

        if !response.status().is_success() {
            tracing::error!("vidlink.pro API returned error: {}", response.status());
            return Err(Error::NotFound(format!(
                "Stream not found for movie ID: {}",
                movie_id
            )));
        }

        // not reading allat but it should be done eventually
        //
        // Parse response
        //
        // something is going on here that won't let the json pass even though it appears valid,
        // issues occur during the mp4 fetch with an exmaple of pluribus ep 4 with something like
        // the prompt/md file
        //
        // oh i also just realized it's zstd encoded no shit it's not happy omg i need to make an
        // http service that will automatically detect and decode. this shit has happened like 3
        // times now where this causes issues fuckkkkk the preview branch is never being worked on
        // again because of all this bullshit.
        //
        // side note that by the time i get to resolving this, hls may change to being the
        // preffered stream and the mp4 will be gone so a new show has to be found that also comes
        // out around this time the best one to try this with s probably going to be one piece or
        // literally anything else that is coming out soon #noticing
        //
        // FIXME:
        let vidlink_response: VidLinkResponse = response.json().await.map_err(|e| {
            tracing::error!("failed to parse vidlink.pro response: {}", e);
            Error::InternalServerErrorWithContext("Failed to parse stream response".to_string())
        })?;

        // the problem could also be here because the stream.playlist should not exist. Instead i
        // would need to fetch stream.qualities.1080.url and make sure that the ../qualities.type
        // is mp4 because confirming the schema.
        //
        // note that this is weird because vidlink doesn't want to play the links anyways
        debug!(
            "received playlist URL: {}",
            vidlink_response.stream.playlist
        );

        let encoded_playlist_url = URL_SAFE
            .encode(vidlink_response.stream.playlist.as_bytes())
            .trim_end_matches('=')
            .to_string();
        let playlist_url = format!("/api/v1/proxy?url={}&schema=movie", encoded_playlist_url);

        let captions: Vec<CaptionInfo> = vidlink_response
            .stream
            .captions
            .into_iter()
            .map(|c| {
                let encoded_caption_url = URL_SAFE
                    .encode(c.url.as_bytes())
                    .trim_end_matches('=')
                    .to_string();
                let proxied_url = format!("/api/v1/proxy/captions?url={}", encoded_caption_url);

                CaptionInfo {
                    language: c.language,
                    url: proxied_url,
                }
            })
            .collect();

        info!(
            "successfully retrieved stream link for movie ID: {} ({})",
            movie_id, content_type
        );

        Ok(GetMovieLinkResponse {
            playlist_url,
            captions,
        })
    }
}
