use serde::{Deserialize, Serialize};
use shared::web::page::PageRequest;
use validator::Validate;

use crate::entities::sea_orm_active_enums::Status;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessKeyPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct AccessKeyInput {
    pub domain: String,
    pub status: Status,
    #[validate(length(max = 200, message = "Description must not exceed 200 characters"))]
    pub description: Option<String>,
}

pub type CreateAccessKeyInput = AccessKeyInput;
