use chrono::NaiveDateTime;
use serde::Serialize;

use crate::model::entity::{sea_orm_active_enums::Status, sys_user};

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserWithoutPassword {
    pub id: String,
    pub domain: String,
    pub username: String,
    pub nick_name: String,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub status: Status,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
}

impl From<sys_user::Model> for UserWithoutPassword {
    fn from(model: sys_user::Model) -> Self {
        Self {
            id: model.id,
            domain: model.domain,
            username: model.username,
            nick_name: model.nick_name,
            avatar: model.avatar,
            email: model.email,
            phone_number: model.phone_number,
            status: model.status,
            created_at: model.created_at,
            created_by: model.created_by,
            updated_at: model.updated_at,
            updated_by: model.updated_by,
        }
    }
}
