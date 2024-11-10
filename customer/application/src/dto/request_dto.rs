use serde::{ Deserialize, Serialize };
use validator::Validate;
#[derive(Default, Debug, Clone, Validate, Serialize, Deserialize)]
pub struct SignInDto {
    #[validate(email)]
    pub email: String,
    pub password: String,
}
// #[derive(Default, Debug, Clone, Validate, Serialize, Deserialize)]
// pub struct UserDTO {
//     #[validate(length(min = 3, max = 20))]
//     pub username: String,
//     #[validate(email)]
//     pub email: String,
//     pub password: String,
// }

// #[derive(Default, Debug, Clone, Validate, Serialize, Deserialize)]
// pub struct LoginInfoDTO {
//     pub jwt: String,
// }

// #[derive(Default, Debug, Clone, Serialize, Deserialize)]
// pub struct TokenClaims {
//     pub sub: String,
//     pub iat: usize,
//     pub exp: usize,
//     pub jti: String, // jwt的唯一标识
// }
#[derive(Default, Debug, Clone, Validate, Serialize, Deserialize)]
pub struct SignUpDTO {
    // 用户名
    pub username: String,
    // 邮箱
    #[validate(email)]
    pub email: String,
    // 密码
    pub password: String,
    // 邮箱验证码
    pub mail_validate_code: String,
}

#[derive(Default, Debug, Clone, Validate, Serialize, Deserialize)]
pub struct VerifyCodeDto {
    //  验证码类型 注册、登录
    pub code_type: String,
    // 接收邮箱
    pub recevive_email: String,
}
#[derive(Default, Debug, Clone, Validate, Serialize, Deserialize)]
pub struct ForgotPasswordRequestDto {
    #[validate(email)]
    pub email: String,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveAddressUpdateDto {
    pub token: String,
    pub new_password: String,
}
