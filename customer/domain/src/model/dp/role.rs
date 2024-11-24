use serde::{ Deserialize, Serialize };
use utoipa::ToSchema;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, ToSchema)]
pub enum Role {
    #[default]
    Admin,
    User,
    System,
}
