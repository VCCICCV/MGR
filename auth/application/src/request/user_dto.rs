use serde::{Deserialize, Serialize};

#[derive(Default,Debug, Clone,Serialize, Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Default,Debug, Clone,Serialize, Deserialize)]
pub struct LoginInfoDTO {
    pub jwt: String,
}

#[derive(Default,Debug, Clone,Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub jti:String,// jwt的唯一标识
}
#[derive(Default,Debug, Clone,Serialize, Deserialize)]
pub struct RegisterUserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Default,Debug, Clone,Serialize, Deserialize)]
pub struct LoginUserDTO {
    pub email: String,
    pub password: String,
}
#[derive(Default,Debug, Clone,Serialize, Deserialize)]
pub struct ForgotPasswordRequestDto {
    pub email: String,
}
#[derive(Default,Debug, Clone,Serialize, Deserialize)]
pub struct ResetPasswordRequestDto {
    pub token: String,
    pub new_password: String,
}
