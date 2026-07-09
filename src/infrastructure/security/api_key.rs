use rand::RngCore;
use sha2::{Digest, Sha256};

const KEY_TAG: &str = "rak";

#[derive(Debug, Clone)]
pub struct GeneratedApiKey {
    /// Public identifier, safe to store/display/search by. Not secret.
    pub prefix: String,
    /// Secret part. Only ever returned once, at creation time.
    pub secret: String,
    /// Full key handed to the user, format: `rak_<prefix>.<secret>`.
    pub full_key: String,
}

pub struct ApiKeySecret;

impl ApiKeySecret {
    /// Generates a new API key: a 12-char hex prefix (identifier) and a
    /// 64-char hex secret (the actual credential).
    pub fn generate() -> GeneratedApiKey {
        let prefix = Self::random_hex(6);
        let secret = Self::random_hex(32);
        let full_key = format!("{KEY_TAG}_{prefix}.{secret}");

        GeneratedApiKey {
            prefix,
            secret,
            full_key,
        }
    }

    /// Hashes a secret using SHA-256, hex encoded. API keys are looked up by
    /// prefix and verified by comparing hashes on every request, so a fast
    /// hash is used here rather than argon2 (which is intentionally slow).
    pub fn hash(secret: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(secret.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn verify(secret: &str, hash: &str) -> bool {
        Self::hash(secret) == hash
    }

    /// Parses a full key of the form `rak_<prefix>.<secret>` into its parts.
    pub fn parse(full_key: &str) -> Option<(String, String)> {
        let rest = full_key.strip_prefix(&format!("{KEY_TAG}_"))?;
        let (prefix, secret) = rest.split_once('.')?;

        if prefix.is_empty() || secret.is_empty() {
            return None;
        }

        Some((prefix.to_string(), secret.to_string()))
    }

    fn random_hex(num_bytes: usize) -> String {
        let mut bytes = vec![0u8; num_bytes];
        rand::rng().fill_bytes(&mut bytes);
        hex::encode(bytes)
    }
}
