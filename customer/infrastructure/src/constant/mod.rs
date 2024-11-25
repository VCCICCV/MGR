use once_cell::sync::Lazy;
use std::time::Duration;
use jsonwebtoken::{ DecodingKey, EncodingKey };
use crate::{
    client::{ builder::ClientBuilder, email::EmailClient, redis::RedisClient },
    config::env::get_env_source,
};
// 这里的常量需要依赖基础设施层，且仅在基础设施使用

// 环境变量前缀
pub const ENV_PREFIX: &str = "APP";
// 配置
pub static CONFIG: Lazy<crate::config::AppConfig> = Lazy::new(||
    crate::config::AppConfig::read(get_env_source(ENV_PREFIX)).unwrap()
);
// redis客户端，由于redis建立链接比数据库快，所以不需要放在appstate中，直接放在全局变量中即可
// 很多时候redis不是必须的，所以为了更灵活的使用，将其放在全局变量中，emial客户端同理
// 如果将redis作为主数据库使用，请在appstate中使用redis
// 这里测试使用常量，实际生产在appstate中使用redis
pub static REDIS: Lazy<RedisClient> = Lazy::new(||
    RedisClient::build_from_config(&CONFIG).unwrap()
);

// email客户端
// 这里存放在appstate中使用
pub static EMAIL: Lazy<EmailClient> = Lazy::new(||
    EmailClient::build_from_config(&CONFIG).unwrap()
);
// Authorization header
pub const AUTHORIZATION: &str = "Authorization";

// 常量
pub const NORMAL_USER: &str = "normal_user";
// 2FA验证码过期时间
pub const EXPIRE_TWO_FACTOR_CODE_SECS: Duration = Duration::from_secs(200);
// Bearer token过期时间
pub const EXPIRE_BEARER_TOKEN_SECS: Duration = Duration::from_secs(600);
// Refresh token过期时间
pub const EXPIRE_REFRESH_TOKEN_SECS: Duration = Duration::from_secs(3600);
// 忘记密码验证码过期时间
pub const EXPIRE_FORGET_PASS_CODE_SECS: Duration = Duration::from_secs(120);
// session过期时间约半小时33.333
pub const EXPIRE_SESSION_CODE_SECS: Duration = Duration::from_secs(2000);