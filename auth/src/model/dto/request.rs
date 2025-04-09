use serde::{ Deserialize, Serialize };
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ActiveRequest {
    pub user_id: Uuid,
    pub code: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Login2faRequest {
    pub user_id: Uuid,
    pub code: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshTokenRequest {
    pub token: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TokenInfoRequest {
    pub token: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ForgetPasswordQueryParam {
    pub email: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SetPasswordRequest {
    pub new_password: String,
    pub code: String,
    pub user_id: Uuid,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProfileRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_2fa: Option<bool>,
    pub is_private: Option<bool>,
}
