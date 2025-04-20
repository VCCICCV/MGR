use serde::Deserialize;

use crate::{database::{DatabaseConfig, DatabasesInstancesConfig}, jwt::JwtConfig, redis::{RedisConfig, RedisInstancesConfig}, server::ServerConfig};
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    /// 主数据库配置
    pub database: DatabaseConfig,

    /// 可选的数据库连接池配置
    pub database_instances: Option<Vec<DatabasesInstancesConfig>>,

    /// HTTP 服务器配置
    pub server: ServerConfig,

    /// JWT 认证配置
    pub jwt: JwtConfig,

    /// 主 Redis 配置
    pub redis: Option<RedisConfig>,

    /// 可选的 Redis 连接池配置
    pub redis_instances: Option<Vec<RedisInstancesConfig>>,
}
