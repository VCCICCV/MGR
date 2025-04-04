use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, ToSchema)]
pub enum Role {
    Admin(i32),
    User(i32),
    System(i32),
}
// 默认为用户
impl Default for Role {
    fn default() -> Self {
        Role::User(2)
    }
}
