// ai generated docs for most of the dtos below, take them with caution and read the api
// implementation instead of just the dtos
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request body for decrypting a movie ID
#[derive(Debug, Deserialize, Validate)]
pub struct DecryptMovieRequest {
    /// URL-safe base64 encoded encrypted movie ID (without padding)
    /// Example: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAoJqtBAkw0H1UB314p1Og5cGACc99c2cZ2cRK4FF2XA"
    #[validate(length(min = 1, message = "encrypted_id cannot be empty"))]
    pub encrypted_id: String,
}

/// response body containing decrypted movie information
#[derive(Debug, Serialize)]
pub struct DecryptMovieResponse {
    /// the decrypted movie ID
    pub movie_id: String,

    /// Unix timestamp (seconds) that was embedded in the encryption
    pub timestamp: u32,

    /// Whether the encryption key is valid (successful decryption)
    pub key_valid: bool,

    /// Human-readable timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_readable: Option<String>,
}

/// Response for key verification endpoint
#[derive(Debug, Serialize)]
pub struct VerifyKeyResponse {
    /// Whether the encryption key is valid
    pub key_valid: bool,

    /// Message describing the key status
    pub message: String,

    /// The encryption key in hex format (for verification)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_hex: Option<String>,
}

/// Request body for encrypting a movie ID
#[derive(Debug, Deserialize, Validate)]
pub struct EncryptMovieRequest {
    /// The movie ID to encrypt
    #[validate(length(min = 1, message = "movie_id cannot be empty"))]
    pub movie_id: String,

    /// Optional timestamp (unix seconds). If not provided, uses current time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u32>,
}

/// Response body containing encrypted movie ID
#[derive(Debug, Serialize)]
pub struct EncryptMovieResponse {
    /// The original movie ID
    pub movie_id: String,

    /// The encrypted movie ID (URL-safe base64, no padding)
    pub encrypted_id: String,

    /// The timestamp that was embedded in the encryption
    pub timestamp: u32,

    /// Human-readable timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_readable: Option<String>,
}

/// Caption information
#[derive(Debug, Serialize, Deserialize)]
pub struct CaptionInfo {
    /// Language of the caption
    pub language: String,

    /// URL to the caption file
    pub url: String,
}

/// Response body for getting movie stream link
#[derive(Debug, Serialize)]
pub struct GetMovieLinkResponse {
    /// URL to the playlist (proxied through our server)
    pub playlist_url: String,

    /// Available captions
    pub captions: Vec<CaptionInfo>,
}

/// vidlink.pro API response structure
#[derive(Debug, Deserialize)]
pub struct VidLinkResponse {
    pub stream: StreamInfo,
}

/// Stream information from vidlink.pro
#[derive(Debug, Deserialize)]
pub struct StreamInfo {
    /// Playlist URL
    pub playlist: String,

    /// Available captions
    #[serde(default)]
    pub captions: Vec<VidLinkCaption>,
}

/// Caption from vidlink.pro API
#[derive(Debug, Deserialize)]
pub struct VidLinkCaption {
    pub url: String,
    pub language: String,
}
