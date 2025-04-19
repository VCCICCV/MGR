use serde::{Deserialize, Serialize};
use shared::res::PageRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationLogPageRequest {
    #[serde(flatten)]
    pub page_details: PageRequest,
    pub keywords: Option<String>,
}
