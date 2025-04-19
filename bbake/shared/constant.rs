use std::time::Duration;

use once_cell::sync::Lazy;

// 这里的常量需要依赖基础设施层，且仅在基础设施使用

// 环境变量前缀
pub const ENV_PREFIX: &str = "APP";
// kafka topic
pub const AUTH_TOPIC: &str = "auth";


// Authorization header
pub const AUTHORIZATION: &str = "Authorization";
// Bearer token
pub const BEARER: &str = "Bearer";
// 常量
pub const NORMAL_USER: &str = "normal_user";
// 注册过期事件
pub const EXPIRE_REGISTER_CODE_SECS: Duration = Duration::from_secs(200);
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
// ttl 2FA过期
pub const CHECK_EMAIL_MESSAGE: &str = "Please check you email.";
// 激活用户码过期时间
pub const EXPIRE_ACTIVE_CODE_SECS: Duration = Duration::from_secs(120);
// 验证码长度
pub const CODE_LEN: usize = 6;