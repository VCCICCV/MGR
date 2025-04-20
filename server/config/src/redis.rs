use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub mode: RedisMode,
    /// Redis 连接 URL
    ///
    /// 支持以下格式：
    /// 1. 标准 TCP 连接:
    ///    redis://[<username>][:<password>@]<hostname>[:port][/[<db>][?protocol=<protocol>]]
    ///    示例：
    ///    - 基本连接：redis://127.0.0.1:6379/0
    ///    - 带密码：redis://:password@127.0.0.1:6379/0
    ///    - 带用户名和密码：redis://username:password@127.0.0.1:6379/0
    ///
    /// 2. Unix Socket 连接 (如果系统支持):
    ///    redis+unix:///<path>[?db=<db>[&pass=<password>][&user=<username>][&protocol=<protocol>]]
    ///    或
    ///    unix:///<path>[?db=<db>][&pass=<password>][&user=<username>][&protocol=<protocol>]]
    pub url: Option<String>,
    /// Redis 集群节点地址列表
    /// 每个地址都支持与 url 相同的格式
    ///
    /// 注意：
    /// - 集群模式下，db 参数将被忽略，因为 Redis 集群不支持多数据库
    /// - 所有节点应使用相同的认证信息（用户名/密码）
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
