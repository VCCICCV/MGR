use serde::{ Deserialize, Serialize };

use shared::web::page::PageRequest;
use validator::Validate;

use crate::entities::sea_orm_active_enums::Status;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserInput {
    pub domain: String,
    #[validate(length(min = 1, max = 50, message = "Username must be between 1 and 50 characters"))]
    pub username: String,
    #[validate(
        length(min = 6, max = 100, message = "Password must be between 6 and 100 characters")
    )]
    pub password: String,
    #[validate(
        length(min = 1, max = 50, message = "Nick name must be between 1 and 50 characters")
    )]
    pub nick_name: String,
    pub avatar: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    #[validate(length(max = 20, message = "Phone number must not exceed 20 characters"))]
    pub phone_number: Option<String>,
    pub status: Status,
}

pub type CreateUserInput = UserInput;

#[derive(Deserialize, Validate)]
pub struct UpdateUserInput {
    pub id: String,
    #[serde(flatten)]
    pub user: UserInput,
}
