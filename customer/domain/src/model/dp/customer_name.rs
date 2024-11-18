use shared::error::DomainError;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserName {
    name: String,
}

impl UserName {
    pub fn new(name: String) -> Result<Self, DomainError> {
        let trimmed_name = name.trim();
        if trimmed_name.is_empty() {
            return Err(DomainError::UserEntityValidationError("用户名不能为空".to_string()));
        } else if trimmed_name.len() < 2 {
            return Err(DomainError::UserEntityValidationError("用户名长度必须大于等于2".to_string()));
        }
        Ok(UserName { name: trimmed_name.to_string() })
    }
}