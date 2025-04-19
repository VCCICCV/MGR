pub use redis::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub mode: RedisMode,
    pub url: Option<String>,
    // 集群模式所有节点使用相同认证信息
    pub urls: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum RedisMode {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "cluster")]
    Cluster,
}
#[derive(Debug, Clone, Deserialize)]
pub struct RedisInstancesConfig {
    pub name: String,
    pub redis: RedisConfig,
}
impl RedisConfig {
    pub fn is_cluster(&self) -> bool {
        self.mode == RedisMode::Cluster
    }

    pub fn get_url(&self) -> Option<String> {
        match self.mode {
            RedisMode::Single => self.url.clone(),
            RedisMode::Cluster => None,
        }
    }

    pub fn get_urls(&self) -> Option<Vec<String>> {
        match self.mode {
            RedisMode::Single => None,
            RedisMode::Cluster => self.urls.clone(),
        }
    }
}
