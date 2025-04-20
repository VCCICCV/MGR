use serde::{Deserialize, Serialize};
use shared::web::page::PageRequest;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct DomainInput {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Code must be between 1 and 50 characters"
    ))]
    pub code: String,
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,
    #[validate(length(max = 500, message = "Description must not exceed 500 characters"))]
    pub description: Option<String>,
}

pub type CreateDomainInput = DomainInput;

#[derive(Deserialize, Validate)]
pub struct UpdateDomainInput {
    pub id: String,
    #[serde(flatten)]
    pub domain: DomainInput,
}
