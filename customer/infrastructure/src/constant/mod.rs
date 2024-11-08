use once_cell::sync::Lazy;

use crate::{client::{builder::ClientBuilder, redis::RedisClient}, config::env::get_env_source};
// 环境变量前缀
pub const ENV_PREFIX: &str = "APP";
// 配置
pub static CONFIG: Lazy<crate::config::AppConfig> = Lazy::new(||
    crate::config::AppConfig::read(get_env_source(ENV_PREFIX)).unwrap()
);
// 客户端
pub static REDIS: Lazy<RedisClient> = Lazy::new(||
    RedisClient::build_from_config(&CONFIG).unwrap()
);
// 常量
pub const NORMAL_USER: &str = "normal_user";