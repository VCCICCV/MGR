pub trait CacheService {
    async fn get(&self, key: &str) -> Option<String>;
    async fn set(&self, key: &str, value: &str, expiration: Option<u64>) -> Result<(), Box<dyn std::error::Error>>;
}