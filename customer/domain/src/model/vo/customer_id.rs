use serde::{Deserialize, Serialize};
use shared::error::DomainError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CustomerId {
    user_id: String,
}

impl CustomerId {
    pub fn new(user_id: String) -> Result<Self, DomainError> {
        if user_id != None {
            Ok(CustomerId { name: trimmed_name.to_string() })
        }
    }
}


