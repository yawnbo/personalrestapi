use hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

pub struct SignatureUtil {
    secret: String,
}

impl SignatureUtil {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    /// sig is based on: user_id + expiry + url + secret
    pub fn generate_signature(&self, user_id: &str, expiry: i64, url: &str) -> String {
        let message = format!("{}{}{}", user_id, expiry, url);

        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .expect("HMAC can take key of any size");

        mac.update(message.as_bytes());

        let result = mac.finalize();
        let code_bytes = result.into_bytes();

        hex::encode(code_bytes)
    }

    pub fn verify_signature(&self, user_id: &str, expiry: i64, url: &str, signature: &str) -> bool {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        if current_time > expiry {
            return false;
        }

        // see if we can regenerate the signature, if we can then it's valid
        let expected_signature = self.generate_signature(user_id, expiry, url);

        signature.as_bytes().len() == expected_signature.as_bytes().len()
            && signature
                .as_bytes()
                .iter()
                .zip(expected_signature.as_bytes().iter())
                .fold(0, |acc, (a, b)| acc | (a ^ b))
                == 0
    }

    pub fn generate_expiry(hours: i64) -> i64 {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        current_time + (hours * 3600)
    }
}

// again, i wrote these up trying to get it done fast and make sure everything works, these should
// be moved into the actual tests folder though FIXME:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_generation() {
        let util = SignatureUtil::new("test_secret".to_string());
        let sig1 = util.generate_signature("user123", 1234567890, "https://example.com");
        let sig2 = util.generate_signature("user123", 1234567890, "https://example.com");

        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_signature_verification() {
        let util = SignatureUtil::new("test_secret".to_string());
        let future_expiry = SignatureUtil::generate_expiry(12);
        let url = "https://example.com";
        let user_id = "user123";

        let signature = util.generate_signature(user_id, future_expiry, url);

        // Valid signature should verify
        assert!(util.verify_signature(user_id, future_expiry, url, &signature));

        // Invalid signature should fail
        assert!(!util.verify_signature(user_id, future_expiry, url, "invalid"));

        // Different user should fail
        assert!(!util.verify_signature("different_user", future_expiry, url, &signature));
    }

    #[test]
    fn test_expired_signature() {
        let util = SignatureUtil::new("test_secret".to_string());
        let past_expiry = 1234567890; // Way in the past
        let url = "https://example.com";
        let user_id = "user123";

        let signature = util.generate_signature(user_id, past_expiry, url);

        // Expired signature should fail even if signature is correct
        assert!(!util.verify_signature(user_id, past_expiry, url, &signature));
    }
}
