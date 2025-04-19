use md5::{Digest, Md5};
use moka::sync::Cache;
use parking_lot::RwLock;
use ring::{digest, hmac};
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

/// 支持API密钥验证的签名算法
///
/// 这些算法用于生成和验证API请求的签名
/// 算法按性能排序（最快到最慢）
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SignatureAlgorithm {
    /// MD5 signature algorithm (default, fastest)
    Md5,
    /// SHA1 signature algorithm
    Sha1,
    /// SHA256 signature algorithm
    Sha256,
    /// HMAC-SHA256 signature algorithm (most secure)
    HmacSha256,
}

impl Default for SignatureAlgorithm {
    #[inline]
    fn default() -> Self {
        Self::Md5
    }
}

///API密钥验证的配置。
///此结构保存API密钥验证系统的配置选项。
///它的设计目的是轻量级且可高效克隆。
#[derive(Debug, Clone, Copy)]
pub struct ApiKeyConfig {
    /// 用于请求验证的签名算法
    pub algorithm: SignatureAlgorithm,
}

impl Default for ApiKeyConfig {
    #[inline]
    fn default() -> Self {
        Self {
            algorithm: SignatureAlgorithm::default(),
        }
    }
}

/// 验证超时和到期的参数
pub const NONCE_TTL_SECS: u64 = 600; // 10 minutes
pub const TIMESTAMP_DISPARITY_MS: i64 = 300_000; // 5 minutes

/// 容量提示
const DEFAULT_CAPACITY: usize = 32;

///内存存储，用于管理自动过期的随机数
///使用moka缓存来存储TTL为10分钟
///过期了，可以重新使用。有助于防止重放攻击
#[derive(Clone)]
pub struct MemoryNonceStore {
    nonces: Cache<String, ()>,
}

impl Default for MemoryNonceStore {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryNonceStore {
    /// 创建具有10分钟TTL的新MemoryNonceStore实例
    #[inline]
    pub fn new() -> Self {
        Self {
            nonces: Cache::builder()
                .time_to_live(std::time::Duration::from_secs(NONCE_TTL_SECS))
                .build(),
        }
    }

    /// Validates and stores a nonce.
    ///
    /// # Arguments
    /// * `nonce` - The nonce string to validate and store
    ///
    /// # Returns
    /// * `true` if the nonce is valid and not previously used
    /// * `false` if the nonce is invalid or has been used before
    #[inline]
    pub fn check_and_set(&self, nonce: &str) -> bool {
        if self.nonces.contains_key(nonce) {
            false
        } else {
            self.nonces.insert(nonce.to_string(), ());
            true
        }
    }
}

///简单的API密钥验证器，用于检查一组预定义的密钥。
///此验证器通过与一组有效密钥进行比较来提供基本的API密钥验证。
///密钥永久存储，只能通过显式API调用进行修改。
#[derive(Clone)]
pub struct SimpleApiKeyValidator {
    keys: Arc<RwLock<HashMap<String, ()>>>,
}

impl SimpleApiKeyValidator {
    /// 使用空的有效密钥集创建新的SimpleApiKeyVeritas
    #[inline]
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::with_capacity(DEFAULT_CAPACITY))),
        }
    }

    /// Validates if an API key is valid.
    ///
    /// # Arguments
    /// * `key` - The API key to validate
    ///
    /// # Returns
    /// * `true` if the key is valid
    /// * `false` if the key is invalid
    #[inline]
    pub fn validate_key(&self, key: &str) -> bool {
        self.keys.read().contains_key(key)
    }

    /// Adds a new valid API key.
    ///
    /// # Arguments
    /// * `key` - The API key to add
    #[inline]
    pub fn add_key(&self, key: String) {
        self.keys.write().insert(key, ());
    }

    /// Removes an API key from the set of valid keys.
    ///
    /// # Arguments
    /// * `key` - The API key to remove
    #[inline]
    pub fn remove_key(&self, key: &str) {
        self.keys.write().remove(key);
    }
}

impl Default for SimpleApiKeyValidator {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

///复杂的API密钥验证器，支持多种签名算法和nonce验证。
///
///此验证器提供高级API密钥验证功能，包括：
/// -多种签名算法（MD5，SHA1，SHA 256，HMAC-SHA 256）
/// -时间戳验证以防止重放攻击
/// -具有自动到期功能的随机数验证
/// - URL参数签名
///
/// API密钥及其相应的秘密永久存储，并且只能
///通过显式API调用修改。
#[derive(Clone)]
pub struct ComplexApiKeyValidator {
    secrets: Arc<RwLock<HashMap<String, String>>>,
    nonce_store: Arc<MemoryNonceStore>,
    config: ApiKeyConfig,
}

impl ComplexApiKeyValidator {
    /// Creates a new ComplexApiKeyValidator with optional configuration.
    ///
    /// # Arguments
    /// * `config` - Optional API key validation configuration. If None, uses default configuration.
    #[inline]
    pub fn new(config: Option<ApiKeyConfig>) -> Self {
        Self {
            secrets: Arc::new(RwLock::new(HashMap::with_capacity(DEFAULT_CAPACITY))),
            nonce_store: Arc::new(MemoryNonceStore::new()),
            config: config.unwrap_or_default(),
        }
    }

    /// Validates if a timestamp is within the allowed 5-minute window.
    #[inline]
    fn validate_timestamp(&self, timestamp: i64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        (now - timestamp).abs() < TIMESTAMP_DISPARITY_MS
    }

    /// Calculates signature for a signing string using the configured algorithm.
    ///
    /// # Arguments
    /// * `signing_string` - The string to sign
    /// * `secret` - The secret key to use for signing
    ///
    /// # Returns
    /// The calculated signature as a hexadecimal string
    #[inline]
    pub fn calculate_signature(&self, signing_string: &str, secret: &str) -> String {
        let signing_string = format!("{}&key={}", signing_string, secret);
        match self.config.algorithm {
            SignatureAlgorithm::Md5 => {
                let mut hasher = Md5::new();
                hasher.update(signing_string.as_bytes());
                hex::encode(hasher.finalize())
            },
            SignatureAlgorithm::Sha1 => {
                let mut context = digest::Context::new(&digest::SHA1_FOR_LEGACY_USE_ONLY);
                context.update(signing_string.as_bytes());
                hex::encode(context.finish())
            },
            SignatureAlgorithm::Sha256 => {
                let mut context = digest::Context::new(&digest::SHA256);
                context.update(signing_string.as_bytes());
                hex::encode(context.finish())
            },
            SignatureAlgorithm::HmacSha256 => {
                let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
                let tag = hmac::sign(&key, signing_string.as_bytes());
                hex::encode(tag.as_ref())
            },
        }
    }

    /// Validates a signed API request.
    ///
    /// # Arguments
    /// * `api_key` - The API key to validate
    /// * `params` - Vector of key-value pairs representing request parameters
    /// * `signature` - The signature to validate
    /// * `timestamp` - Request timestamp in milliseconds since UNIX epoch
    /// * `nonce` - Unique request identifier to prevent replay attacks
    ///
    /// # Returns
    /// * `true` if the request is valid
    /// * `false` if any validation check fails
    pub fn validate_signature(
        &self,
        api_key: &str,
        params: &[(String, String)],
        signature: &str,
        timestamp: i64,
        nonce: &str,
    ) -> bool {
        if !self.validate_timestamp(timestamp) {
            return false;
        }

        if !self.nonce_store.check_and_set(nonce) {
            return false;
        }

        let secrets_guard = self.secrets.read();
        let secret = match secrets_guard.get(api_key) {
            Some(s) => Cow::Borrowed(s),
            None => return false,
        };

        // Pre-allocate with capacity to avoid reallocations
        let mut sorted_params: Vec<_> = Vec::with_capacity(params.len());
        sorted_params.extend_from_slice(params);
        sorted_params.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        // Pre-calculate total length to avoid reallocations
        let total_len = sorted_params.iter().fold(0, |acc, (k, v)| {
            acc + k.len() + v.len() + 2 // +2 for '=' and '&'
        });

        let mut signing_string = String::with_capacity(total_len);
        for (i, (k, v)) in sorted_params.iter().enumerate() {
            if i > 0 {
                signing_string.push('&');
            }
            signing_string.push_str(k);
            signing_string.push('=');
            // Only URL encode if necessary
            if v.chars().any(|c| !c.is_ascii_alphanumeric()) {
                signing_string.push_str(&urlencoding::encode(v));
            } else {
                signing_string.push_str(v);
            }
        }

        self.calculate_signature(&signing_string, &secret) == signature
    }

    /// Adds a new API key and its corresponding secret.
    ///
    /// # Arguments
    /// * `key` - The API key to add
    /// * `secret` - The secret corresponding to the API key
    #[inline]
    pub fn add_key_secret(&self, key: String, secret: String) {
        self.secrets.write().insert(key, secret);
    }

    /// Removes an API key and its secret.
    ///
    /// # Arguments
    /// * `key` - The API key to remove
    #[inline]
    pub fn remove_key(&self, key: &str) {
        self.secrets.write().remove(key);
    }

    /// Updates the API key validation configuration.
    ///
    /// # Arguments
    /// * `config` - New API key validation configuration
    #[inline]
    pub fn update_config(&mut self, config: ApiKeyConfig) {
        self.config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_simple_api_key_validator() {
        let validator = SimpleApiKeyValidator::new();
        validator.add_key("test-key".to_string());

        assert!(validator.validate_key("test-key"));
        assert!(!validator.validate_key("invalid-key"));
    }

    #[test]
    fn test_nonce_store() {
        let store = MemoryNonceStore::new();
        assert!(store.check_and_set("nonce1"));
        assert!(!store.check_and_set("nonce1"));
        thread::sleep(std::time::Duration::from_secs(NONCE_TTL_SECS + 1));
        assert!(store.check_and_set("nonce1"));
    }

    #[test]
    fn test_complex_validator() {
        let validator = ComplexApiKeyValidator::new(Some(ApiKeyConfig {
            algorithm: SignatureAlgorithm::Md5,
        }));

        validator.add_key_secret("test-key".to_string(), "test-secret".to_string());

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let params = vec![
            ("timestamp".to_string(), now.to_string()),
            ("nonce".to_string(), "test-nonce".to_string()),
            ("data".to_string(), "test-data".to_string()),
        ];

        let signing_string = format!("data=test-data&nonce=test-nonce&timestamp={}", now);
        let signature = validator.calculate_signature(&signing_string, "test-secret");

        assert!(validator.validate_signature("test-key", &params, &signature, now, "test-nonce"));
    }

    #[test]
    fn test_concurrent_access() {
        let validator = Arc::new(ComplexApiKeyValidator::new(None));
        let mut handles = Vec::new();

        for i in 0..10 {
            let validator = validator.clone();
            let handle = thread::spawn(move || {
                let key = format!("key{}", i);
                let secret = format!("secret{}", i);
                validator.add_key_secret(key.clone(), secret.clone());
                assert!(validator.secrets.read().contains_key(&key));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
