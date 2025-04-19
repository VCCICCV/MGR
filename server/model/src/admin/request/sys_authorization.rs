use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AssignPermissionDto {
    #[validate(length(min = 1, message = "domain cannot be empty"))]
    pub domain: String,

    #[validate(length(min = 1, message = "Role ID cannot be empty"))]
    pub role_id: String,

    #[validate(length(min = 1, message = "Permissions array cannot be empty"))]
    pub permissions: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AssignRouteDto {
    #[validate(length(min = 1, message = "domain cannot be empty"))]
    pub domain: String,

    #[validate(length(min = 1, message = "Role ID cannot be empty"))]
    pub role_id: String,

    #[validate(length(min = 1, message = "Routes array cannot be empty"))]
    pub route_ids: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AssignUserDto {
    #[validate(length(min = 1, message = "Role ID cannot be empty"))]
    pub role_id: String,

    #[validate(length(min = 1, message = "Users array cannot be empty"))]
    pub user_ids: Vec<String>,
}
