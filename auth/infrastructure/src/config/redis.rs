pub use redis::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    port: u16,
    host: String,
}

impl RedisConfig {
    // 获取redis连接地址
    pub fn get_url(&self) -> String {
        format!(
            "redis://{host}:{port}",
            host = self.host,
            port = self.port,
        )
    }
}
