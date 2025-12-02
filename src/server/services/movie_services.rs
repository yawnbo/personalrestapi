use std::sync::Arc;

use base64::{Engine, engine::general_purpose::URL_SAFE};
use chrono::{DateTime, Utc};
use crypto_secretbox::{
    XSalsa20Poly1305,
    aead::{Aead, KeyInit},
};
use tracing::{debug, error};

use crate::server::{
    dtos::movie_dto::{DecryptMovieResponse, EncryptMovieResponse, VerifyKeyResponse},
    error::{AppResult, Error},
};

pub type DynMovieService = Arc<dyn MovieServiceTrait + Send + Sync>;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait MovieServiceTrait {
    /// decrypt movie id and get timestamp
    async fn decrypt_movie_id(&self, encrypted_id: &str) -> AppResult<DecryptMovieResponse>;

    /// opposite of above
    async fn encrypt_movie_id(
        &self,
        movie_id: &str,
        timestamp: Option<u32>,
    ) -> AppResult<EncryptMovieResponse>;

    /// verify validity
    async fn verify_key(&self) -> AppResult<VerifyKeyResponse>;
}

pub struct MovieService {
    /// IMPORTANT: THIS IS NOT A SECRET KEY it is embedded in client sidded (NOT MINE I WOULDN'T DO
    /// THIS EVER!!!!) wasm for obfuscation purposes only and is PUBLIC knowledge
    ///
    /// 32-byte key WASM
    /// hex: c75136c5668bbfe65a7ecad431a745db68b5f381555b38d8f6c699449cf11fcd
    /// b64: x1E2xWaLv+ZafsrUMadF22i184FVWzjY9saZRJzxH80=
    encryption_key: [u8; 32],
}

impl MovieService {
    pub fn new() -> Self {
        // encryption key extracted from fu.wasm (Oct 21, 2025)
        //
        // update: they haven't changed it as of Dec 1, 2025 so i don't think they cycle keys
        // unless it's yearly or something
        let encryption_key: [u8; 32] = [
            0xc7, 0x51, 0x36, 0xc5, 0x66, 0x8b, 0xbf, 0xe6, 0x5a, 0x7e, 0xca, 0xd4, 0x31, 0xa7,
            0x45, 0xdb, 0x68, 0xb5, 0xf3, 0x81, 0x55, 0x5b, 0x38, 0xd8, 0xf6, 0xc6, 0x99, 0x44,
            0x9c, 0xf1, 0x1f, 0xcd,
        ];

        Self { encryption_key }
    }

    /// Decrypt using NaCl secretbox (XSalsa20-Poly1305) as per the wasm implementation
    /// get better security yo maybe don't do it client side !!!!!!!!!!!
    fn decrypt_secretbox(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, Error> {
        // first 24 bytes are the nonce (all zeros in this implementation)
        if encrypted_data.len() < 24 {
            return Err(Error::BadRequest(
                "Encrypted data too short (need at least 24 bytes for nonce)".to_string(),
            ));
        }

        let nonce_bytes = &encrypted_data[0..24];
        let ciphertext = &encrypted_data[24..];

        let mut nonce = [0u8; 24];
        nonce.copy_from_slice(nonce_bytes);

        // now that i think about it this should be probably be a service so i'm not making an
        // instance every time..... this code is shit
        // FIXME:
        let cipher = XSalsa20Poly1305::new(&self.encryption_key.into());

        let plaintext = cipher
            .decrypt(&nonce.into(), ciphertext)
            .map_err(|_| Error::BadRequest("Decryption failed - invalid ciphertext".to_string()))?;

        Ok(plaintext)
    }

    /// parse plaintext to extract movie ID and timestamp
    /// format: [movie_id_string] + [4_null_bytes] + [unix_timestamp_4_bytes_big_endian]
    fn parse_plaintext(&self, plaintext: &[u8]) -> Result<(String, u32), Error> {
        // find last pos
        let null_pos = plaintext.iter().position(|&b| b == 0).ok_or_else(|| {
            Error::BadRequest("Invalid plaintext format (no null terminator)".to_string())
        })?;

        let movie_id = String::from_utf8(plaintext[0..null_pos].to_vec())
            .map_err(|_| Error::BadRequest("Invalid movie ID encoding".to_string()))?;

        // skip the padding
        let timestamp_start = null_pos + 4;
        if plaintext.len() < timestamp_start + 4 {
            return Err(Error::BadRequest(
                "Invalid plaintext format (missing timestamp)".to_string(),
            ));
        }

        let timestamp_bytes = &plaintext[timestamp_start..timestamp_start + 4];
        let timestamp = u32::from_be_bytes([
            timestamp_bytes[0],
            timestamp_bytes[1],
            timestamp_bytes[2],
            timestamp_bytes[3],
        ]);

        Ok((movie_id, timestamp))
    }

    fn encrypt_secretbox(&self, plaintext: &[u8]) -> Result<Vec<u8>, Error> {
        let nonce = [0u8; 24];

        let cipher = XSalsa20Poly1305::new(&self.encryption_key.into());

        let ciphertext = cipher
            .encrypt(&nonce.into(), plaintext)
            .map_err(|_| Error::InternalServerErrorWithContext("Encryption failed".to_string()))?;

        let mut full_output = Vec::with_capacity(24 + ciphertext.len());
        full_output.extend_from_slice(&nonce);
        full_output.extend_from_slice(&ciphertext);

        Ok(full_output)
    }

    fn build_plaintext(&self, movie_id: &str, timestamp: u32) -> Vec<u8> {
        let movie_id_bytes = movie_id.as_bytes();
        let null_padding = [0u8; 4];
        let timestamp_bytes = timestamp.to_be_bytes();

        let mut plaintext = Vec::with_capacity(movie_id_bytes.len() + 4 + 4);
        plaintext.extend_from_slice(movie_id_bytes);
        plaintext.extend_from_slice(&null_padding);
        plaintext.extend_from_slice(&timestamp_bytes);

        plaintext
    }
}

impl Default for MovieService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl MovieServiceTrait for MovieService {
    async fn decrypt_movie_id(&self, encrypted_id: &str) -> AppResult<DecryptMovieResponse> {
        debug!("Decrypting movie ID: {}", encrypted_id);

        let padding_needed = (4 - encrypted_id.len() % 4) % 4;
        let padded = format!("{}{}", encrypted_id, "=".repeat(padding_needed));

        let encrypted_data = URL_SAFE
            .decode(&padded)
            .map_err(|e| Error::BadRequest(format!("Invalid base64 encoding: {}", e)))?;

        debug!("Decoded {} bytes from base64", encrypted_data.len());

        let plaintext = self.decrypt_secretbox(&encrypted_data)?;

        debug!("Decrypted {} bytes of plaintext", plaintext.len());

        let (movie_id, timestamp) = self.parse_plaintext(&plaintext)?;

        let datetime = DateTime::<Utc>::from_timestamp(timestamp as i64, 0);
        let timestamp_readable = datetime.map(|dt| dt.to_rfc3339());

        debug!(
            "Successfully decrypted movie ID: {}, timestamp: {}",
            movie_id, timestamp
        );

        Ok(DecryptMovieResponse {
            movie_id,
            timestamp,
            key_valid: true,
            timestamp_readable,
        })
    }

    async fn encrypt_movie_id(
        &self,
        movie_id: &str,
        timestamp: Option<u32>,
    ) -> AppResult<EncryptMovieResponse> {
        debug!("Encrypting movie ID: {}", movie_id);

        let timestamp = if let Some(ts) = timestamp {
            ts
        } else {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| {
                    Error::InternalServerErrorWithContext(
                        "System time before UNIX epoch".to_string(),
                    )
                })?
                .as_secs() as u32
        };

        let plaintext = self.build_plaintext(movie_id, timestamp);

        debug!("Built plaintext of {} bytes", plaintext.len());

        let encrypted_data = self.encrypt_secretbox(&plaintext)?;

        debug!("Encrypted to {} bytes", encrypted_data.len());

        let encrypted_id = URL_SAFE
            .encode(&encrypted_data)
            .trim_end_matches('=')
            .to_string();

        let datetime = DateTime::<Utc>::from_timestamp(timestamp as i64, 0);
        let timestamp_readable = datetime.map(|dt| dt.to_rfc3339());

        debug!(
            "Successfully encrypted movie ID: {}, timestamp: {}",
            movie_id, timestamp
        );

        Ok(EncryptMovieResponse {
            movie_id: movie_id.to_string(),
            encrypted_id,
            timestamp,
            timestamp_readable,
        })
    }

    async fn verify_key(&self) -> AppResult<VerifyKeyResponse> {
        debug!("Verifying encryption key");

        // test with known sample
        // movie ID: 1311031, timestamp: 1761037963
        // expected encrypted: AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAoJqtBAkw0H1UB314p1Og5cGACc99c2cZ2cRK4FF2XA
        let test_encrypted =
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAoJqtBAkw0H1UB314p1Og5cGACc99c2cZ2cRK4FF2XA";

        match self.decrypt_movie_id(test_encrypted).await {
            Ok(result) => {
                if result.movie_id == "1311031" && result.timestamp == 1761037963 {
                    debug!("Key verification successful");
                    Ok(VerifyKeyResponse {
                        key_valid: true,
                        message: "Encryption key is valid and working correctly".to_string(),
                        key_hex: Some(hex::encode(self.encryption_key)),
                    })
                } else {
                    error!(
                        "Key verification failed: unexpected values (movie_id: {}, timestamp: {})",
                        result.movie_id, result.timestamp
                    );
                    Ok(VerifyKeyResponse {
                        key_valid: false,
                        message:
                            "Key decryption succeeded but values don't match expected test case"
                                .to_string(),
                        key_hex: None,
                    })
                }
            }
            Err(e) => {
                error!("Key verification failed: {}", e);
                Ok(VerifyKeyResponse {
                    key_valid: false,
                    message: format!("Key verification failed: {}", e),
                    key_hex: None,
                })
            }
        }
    }
}

// i wrote these up because i wanted to be done with it and just test it quickly so i put them here
// but they should REALLY be in the tests foler made literally for this
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_decrypt_known_sample_1() {
        let service = MovieService::new();
        let encrypted =
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAoJqtBAkw0H1UB314p1Og5cGACc99c2cZ2cRK4FF2XA";

        let result = service.decrypt_movie_id(encrypted).await.unwrap();

        assert_eq!(result.movie_id, "1311031");
        assert_eq!(result.timestamp, 1761037963);
        assert!(result.key_valid);
    }

    #[tokio::test]
    async fn test_decrypt_known_sample_2() {
        let service = MovieService::new();
        let encrypted = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAH9bwQ0Zdid-ohCscvgLYNcaCD89_dlYZ2cQif-vm";

        let result = service.decrypt_movie_id(encrypted).await.unwrap();

        assert_eq!(result.movie_id, "617126");
        assert!(result.key_valid);
    }

    #[tokio::test]
    async fn test_verify_key() {
        let service = MovieService::new();

        let result = service.verify_key().await.unwrap();

        assert!(result.key_valid);
        assert!(result.key_hex.is_some());
    }

    #[tokio::test]
    async fn test_decrypt_invalid_base64() {
        let service = MovieService::new();
        let invalid = "not-valid-base64!!!";

        let result = service.decrypt_movie_id(invalid).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_decrypt_wrong_key() {
        // test with random data to make sure it fails
        let service = MovieService::new();
        let random_base64 = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA" // 24 byte nonce
            .to_string()
            + "RANDOM";

        let result = service.decrypt_movie_id(&random_base64).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_encrypt_movie_id() {
        let service = MovieService::new();
        let movie_id = "1311031";
        let timestamp = 1761037963;

        let result = service
            .encrypt_movie_id(movie_id, Some(timestamp))
            .await
            .unwrap();

        assert_eq!(result.movie_id, movie_id);
        assert_eq!(result.timestamp, timestamp);
        assert!(!result.encrypted_id.is_empty());
        assert!(result.timestamp_readable.is_some());
    }

    #[tokio::test]
    async fn test_encrypt_then_decrypt() {
        let service = MovieService::new();
        let movie_id = "1311031";
        let timestamp = 1761037963;

        let encrypt_result = service
            .encrypt_movie_id(movie_id, Some(timestamp))
            .await
            .unwrap();

        let decrypt_result = service
            .decrypt_movie_id(&encrypt_result.encrypted_id)
            .await
            .unwrap();

        assert_eq!(decrypt_result.movie_id, movie_id);
        assert_eq!(decrypt_result.timestamp, timestamp);
        assert!(decrypt_result.key_valid);
    }

    #[tokio::test]
    async fn test_encrypt_matches_known_sample() {
        let service = MovieService::new();
        let movie_id = "1311031";
        let timestamp = 1761037963;

        let result = service
            .encrypt_movie_id(movie_id, Some(timestamp))
            .await
            .unwrap();

        assert_eq!(
            result.encrypted_id,
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAoJqtBAkw0H1UB314p1Og5cGACc99c2cZ2cRK4FF2XA"
        );
    }
}
