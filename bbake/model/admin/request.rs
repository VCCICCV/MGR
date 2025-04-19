use serde::Deserialize;
use validator::Validate;

use crate::model::entity::sea_orm_active_enums::Status;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserInput {
    pub domain: String,

    pub username: String,

    pub password: String,

    pub nick_name: String,
    pub avatar: Option<String>,

    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub status: Status,
}