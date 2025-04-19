use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginInput {
    #[validate(length(min = 5, message = "Username cannot be empty"))]
    pub identifier: String,
    #[validate(length(min = 6, message = "Password cannot be empty"))]
    pub password: String,
}
