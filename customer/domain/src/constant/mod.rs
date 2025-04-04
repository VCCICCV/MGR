use std::time::Duration;
// session过期时间
pub const EXPIRE_SESSION_CODE_SECS: Duration = Duration::from_secs(2000);
// 激活用户码过期时间
pub const EXPIRE_ACTIVE_CODE_SECS: Duration = Duration::from_secs(120);
// 忘记密码验证码过期时间
pub const EXPIRE_FORGET_PASS_CODE_SECS: Duration = Duration::from_secs(100);
// 2FA验证码过期时间
pub const EXPIRE_TWO_FACTOR_CODE_SECS: Duration = Duration::from_secs(200);
// ttl 2FA过期
pub const CHECK_EMAIL_MESSAGE: &str = "Please check you email.";
// // 验证码长度
pub const CODE_LEN: usize = 6;
