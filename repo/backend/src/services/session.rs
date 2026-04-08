use chrono::NaiveDateTime;
use hmac::{Hmac, Mac};
use rand::Rng;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Configuration for cookie-based session management.
pub struct SessionConfig {
    pub cookie_secret: [u8; 32],
    /// Idle timeout in seconds (default 1800 = 30 min).
    pub idle_timeout_secs: u64,
    /// How often session IDs are rotated (default 300 = 5 min).
    pub rotation_interval_secs: u64,
}

impl SessionConfig {
    /// Build from the `COOKIE_SECRET` env var (or a default dev secret).
    /// The raw value is hashed with SHA-256 to produce a 32-byte key.
    pub fn from_env() -> Self {
        let raw = std::env::var("COOKIE_SECRET")
            .unwrap_or_else(|_| "brewflow-dev-cookie-secret".into());

        use sha2::Digest;
        let mut hasher = sha2::Sha256::new();
        hasher.update(raw.as_bytes());
        let hash: [u8; 32] = hasher.finalize().into();

        SessionConfig {
            cookie_secret: hash,
            idle_timeout_secs: 1800,
            rotation_interval_secs: 300,
        }
    }
}

/// Generate a cryptographically-random 32-byte hex session ID.
pub fn create_session_id() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill(&mut bytes);
    hex::encode(bytes)
}

/// Sign a session ID with HMAC-SHA256 and return `session_id.signature`.
pub fn sign_cookie(config: &SessionConfig, session_id: &str) -> String {
    let mut mac =
        HmacSha256::new_from_slice(&config.cookie_secret).expect("HMAC accepts any key length");
    mac.update(session_id.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());
    format!("{}.{}", session_id, signature)
}

/// Verify a signed cookie value (`session_id.signature`).
/// Returns the session_id if the HMAC is valid, `None` otherwise.
pub fn verify_cookie(config: &SessionConfig, cookie_value: &str) -> Option<String> {
    let (session_id, signature) = cookie_value.rsplit_once('.')?;
    let sig_bytes = hex::decode(signature).ok()?;

    let mut mac =
        HmacSha256::new_from_slice(&config.cookie_secret).expect("HMAC accepts any key length");
    mac.update(session_id.as_bytes());
    mac.verify_slice(&sig_bytes).ok()?;

    Some(session_id.to_string())
}

/// Returns `true` if the session should be rotated (i.e. `last_rotated` is
/// older than `rotation_interval_secs`).
pub fn should_rotate(last_rotated: NaiveDateTime, config: &SessionConfig) -> bool {
    let now = chrono::Utc::now().naive_utc();
    let elapsed = now.signed_duration_since(last_rotated);
    elapsed.num_seconds() as u64 >= config.rotation_interval_secs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> SessionConfig {
        SessionConfig {
            cookie_secret: [42u8; 32],
            idle_timeout_secs: 1800,
            rotation_interval_secs: 300,
        }
    }

    #[test]
    fn sign_then_verify_round_trip() {
        let config = test_config();
        let session_id = "deadbeef1234567890abcdef";
        let signed = sign_cookie(&config, session_id);
        let recovered = verify_cookie(&config, &signed);
        assert_eq!(recovered, Some(session_id.to_string()));
    }

    #[test]
    fn verify_tampered_signature_fails() {
        let config = test_config();
        let signed = sign_cookie(&config, "some-session-id");
        // Flip the last character of the signature.
        let mut tampered = signed.clone();
        let last = tampered.pop().unwrap();
        tampered.push(if last == 'a' { 'b' } else { 'a' });
        assert_eq!(verify_cookie(&config, &tampered), None);
    }

    #[test]
    fn verify_wrong_key_fails() {
        let config1 = test_config();
        let config2 = SessionConfig {
            cookie_secret: [7u8; 32],
            idle_timeout_secs: 1800,
            rotation_interval_secs: 300,
        };
        let signed = sign_cookie(&config1, "abc");
        assert_eq!(verify_cookie(&config2, &signed), None);
    }

    #[test]
    fn verify_missing_dot_separator_fails() {
        let config = test_config();
        assert_eq!(verify_cookie(&config, "nodothere"), None);
    }

    #[test]
    fn should_rotate_old_session() {
        let config = test_config();
        // A timestamp well in the past should require rotation.
        let old = chrono::Utc::now().naive_utc() - chrono::Duration::seconds(600);
        assert!(should_rotate(old, &config));
    }

    #[test]
    fn should_not_rotate_recent_session() {
        let config = test_config();
        // A timestamp from 10 seconds ago should NOT require rotation.
        let recent = chrono::Utc::now().naive_utc() - chrono::Duration::seconds(10);
        assert!(!should_rotate(recent, &config));
    }
}
