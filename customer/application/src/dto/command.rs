use serde::{ Deserialize, Serialize };
use garde::Validate;
use utoipa::{ IntoParams, ToSchema };
use uuid::Uuid;

// 命令
pub enum Command {
    SignIn(SignInCommand),
    SignUp(SignUpCommand),
}
// 重置密码请求
#[derive(Debug, Deserialize, Serialize, ToSchema, Validate, IntoParams)]
pub struct SetPasswordCommand {
    #[garde(length(min = 8))]
    pub new_password: String,
    #[garde(length(min = 5))]
    pub code: String,
    #[garde(skip)]
    pub user_id: Uuid,
}
// 2fa登录请求
#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct SignIn2FaCommand {
    #[garde(skip)]
    pub user_id: Uuid,
    #[garde(length(min = 5))]
    pub code: String,
}
// 刷新token请求
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, IntoParams)]
pub struct RefreshTokenCommand {
    #[garde(length(min = 30))]
    pub token: String,
}
// 使用token请求
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, IntoParams)]
pub struct TokenInfoCommand {
    #[garde(length(min = 30))]
    pub token: String,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct VerifyCodeSendCommand {
    //  验证码类型 注册、登录
    pub code_type: String,
    // 接收邮箱
    pub receive_email: String,
}
// 激活命令
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema, IntoParams)]
pub struct ActiveCommand {
    #[garde(length(min = 5))]
    pub verify_code: String,
    #[garde(skip)]
    pub user_id: Uuid,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate, ToSchema, IntoParams)]
pub struct SignInCommand {
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 6, max = 24))]
    pub password: String,
}

impl SignInCommand {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}
#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate, ToSchema, IntoParams)]
pub struct SignUpCommand {
    // 用户名
    #[garde(length(min = 3, max = 25))]
    pub username: String,
    // 邮箱
    #[garde(email)]
    pub email: String,
    // 密码
    #[garde(length(min = 8))]
    pub password: String,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_email_register_request() {
        let req = SignInCommand::new("email", "password");
        assert!(req.validate().is_err());
    }
}
