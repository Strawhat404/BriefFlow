use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::fmt;

/// Errors that can occur during cryptographic operations.
#[derive(Debug)]
pub enum CryptoError {
    InvalidKey,
    DecryptionFailed,
    InvalidFormat,
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::InvalidKey => write!(f, "Invalid encryption key"),
            CryptoError::DecryptionFailed => write!(f, "Decryption failed"),
            CryptoError::InvalidFormat => write!(f, "Invalid ciphertext format"),
        }
    }
}

impl std::error::Error for CryptoError {}

/// Configuration for AES-256-GCM encryption.
pub struct CryptoConfig {
    pub encryption_key: [u8; 32],
}

impl CryptoConfig {
    /// Build a `CryptoConfig` from the `ENCRYPTION_KEY` env var (or a default
    /// dev key).  The raw env value is hashed with SHA-256 to produce a
    /// deterministic 32-byte key.
    pub fn from_env() -> Self {
        let raw = std::env::var("ENCRYPTION_KEY")
            .unwrap_or_else(|_| "brewflow-dev-encryption-key".into());

        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        let hash = hasher.finalize();

        let mut encryption_key = [0u8; 32];
        encryption_key.copy_from_slice(&hash);

        CryptoConfig { encryption_key }
    }
}

/// Encrypt `plaintext` with AES-256-GCM.  A random 12-byte nonce is prepended
/// to the ciphertext and the whole blob is base64-encoded.
pub fn encrypt(config: &CryptoConfig, plaintext: &str) -> String {
    let key = Key::<Aes256Gcm>::from_slice(&config.encryption_key);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .expect("AES-GCM encryption should not fail");

    let mut combined = Vec::with_capacity(12 + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    B64.encode(combined)
}

/// Decrypt a value previously produced by [`encrypt`].
pub fn decrypt(config: &CryptoConfig, ciphertext_b64: &str) -> Result<String, CryptoError> {
    let combined = B64.decode(ciphertext_b64).map_err(|_| CryptoError::InvalidFormat)?;

    if combined.len() < 13 {
        return Err(CryptoError::InvalidFormat);
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let key = Key::<Aes256Gcm>::from_slice(&config.encryption_key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::DecryptionFailed)?;

    String::from_utf8(plaintext).map_err(|_| CryptoError::DecryptionFailed)
}

/// Mask a value for logging, showing only the first `visible_chars` characters
/// followed by `***`.  E.g. `mask_for_log("BF-A3XYZ", 4)` -> `"BF-A***"`.
pub fn mask_for_log(value: &str, visible_chars: usize) -> String {
    if value.len() <= visible_chars {
        return format!("{}***", value);
    }
    let visible: String = value.chars().take(visible_chars).collect();
    format!("{}***", visible)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> CryptoConfig {
        CryptoConfig {
            encryption_key: [0xABu8; 32],
        }
    }

    // ── encrypt / decrypt ──────────────────────────────────────────────────

    #[test]
    fn encrypt_decrypt_round_trip() {
        let config = test_config();
        let plaintext = "BF-ABC123";
        let ciphertext = encrypt(&config, plaintext);
        let recovered = decrypt(&config, &ciphertext).unwrap();
        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn encrypt_produces_different_ciphertext_each_call() {
        let config = test_config();
        let c1 = encrypt(&config, "hello");
        let c2 = encrypt(&config, "hello");
        // Random nonce means different ciphertext every time.
        assert_ne!(c1, c2);
    }

    #[test]
    fn decrypt_wrong_key_fails() {
        let config1 = test_config();
        let config2 = CryptoConfig {
            encryption_key: [0x11u8; 32],
        };
        let ciphertext = encrypt(&config1, "secret");
        assert!(decrypt(&config2, &ciphertext).is_err());
    }

    #[test]
    fn decrypt_invalid_base64_fails() {
        let config = test_config();
        assert!(matches!(decrypt(&config, "!!!notbase64"), Err(CryptoError::InvalidFormat)));
    }

    #[test]
    fn decrypt_too_short_fails() {
        let config = test_config();
        // 12 bytes of nonce minimum + at least 1 byte ciphertext; fewer bytes → InvalidFormat.
        let short = base64::engine::general_purpose::STANDARD.encode([0u8; 5]);
        assert!(decrypt(&config, &short).is_err());
    }

    // ── mask_for_log ───────────────────────────────────────────────────────

    #[test]
    fn mask_hides_suffix() {
        assert_eq!(mask_for_log("BF-A3XYZ", 4), "BF-A***");
    }

    #[test]
    fn mask_short_value_shows_all_plus_stars() {
        assert_eq!(mask_for_log("AB", 4), "AB***");
    }

    #[test]
    fn mask_exact_length_shows_all_plus_stars() {
        assert_eq!(mask_for_log("ABCD", 4), "ABCD***");
    }

    #[test]
    fn mask_zero_visible() {
        assert_eq!(mask_for_log("hello", 0), "***");
    }
}
