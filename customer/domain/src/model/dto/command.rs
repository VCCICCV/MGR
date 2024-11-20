use serde::{ Deserialize, Serialize };
use garde::Validate;
use uuid::Uuid;
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct VerifyCodeSendCommand {
    //  验证码类型 注册、登录
    pub code_type: String,
    // 接收邮箱
    pub receive_email: String,
}
// 激活命令
#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ActiveCommand {
    #[garde(length(min = 5))]
    pub verify_code: String,
    #[garde(skip)]
    pub user_id: Uuid,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
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
#[derive(Default, Debug, Clone, Serialize, Deserialize, Validate)]
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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct UserLoginRespCommand {
    // 用户名
    pub username: String,
    // 邮箱
    pub email: String,
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
