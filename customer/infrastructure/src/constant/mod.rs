use once_cell::sync::Lazy;

use crate::{ client::{ builder::ClientBuilder, email::EmailClient, redis::RedisClient }, config::env::get_env_source };
// 环境变量前缀
pub const ENV_PREFIX: &str = "APP";
// 配置
pub static CONFIG: Lazy<crate::config::AppConfig> = Lazy::new(||
    crate::config::AppConfig::read(get_env_source(ENV_PREFIX)).unwrap()
);
// redis客户端，由于redis建立链接比数据库快，所以不需要放在appstate中，直接放在全局变量中即可
// 很多时候redis不是必须的，所以为了更灵活的使用，将其放在全局变量中，emial客户端同理
// 如果将redis作为主数据库使用，请在appstate中使用redis
pub static REDIS: Lazy<RedisClient> = Lazy::new(||
    RedisClient::build_from_config(&CONFIG).unwrap()
);
// 常量
pub const NORMAL_USER: &str = "normal_user";
// email客户端
pub static EMAIL: Lazy<EmailClient> = Lazy::new(||
    EmailClient::build_from_config(&CONFIG).unwrap()
);
