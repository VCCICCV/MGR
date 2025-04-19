use std::time::Duration;

use strum::{AsRefStr, Display, EnumString};

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

/// 不同平台受众
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Audience {
    /// 官网
    OfficialWebsite,
    /// 管理平台
    ManagementPlatform,
    /// 手机
    MobileApp,
    /// 小程序
    MiniProgram,
}

impl Audience {
    /// 返回与每个平台关联的受众字符串
    pub fn as_str(self) -> &'static str {
        match self {
            Audience::OfficialWebsite => "official_website",
            Audience::ManagementPlatform => "management_platform",
            Audience::MobileApp => "mobile_app",
            Audience::MiniProgram => "mini_program",
        }
    }
}
/// 系统事件类型枚举
#[derive(Debug, Clone, PartialEq, Eq, AsRefStr, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum SystemEvent {
    /// 用户认证登录事件
    AuthLoggedInEvent,
    /// 系统操作日志事件
    AuditOperationLoggedEvent,
    /// API密钥验证事件
    AuthApiKeyValidatedEvent,
}
/// Token 状态枚举
#[derive(Debug, Clone, PartialEq, Eq, AsRefStr, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenStatus {
    /// 活跃状态，可以正常使用
    Active,
    /// 已被刷新，表示该 token 已被新 token 替换
    Refreshed,
    /// 已被撤销（手动注销或安全原因）
    Revoked,
}

impl TokenStatus {
    pub fn is_valid(&self) -> bool {
        matches!(self, TokenStatus::Active)
    }

    pub fn can_refresh(&self) -> bool {
        matches!(self, TokenStatus::Active)
    }
}
