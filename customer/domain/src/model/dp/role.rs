use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord,ToSchema)]
pub enum Role {
    Admin,
    User,
    System,
}
