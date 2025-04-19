use server_core::web::error::{ApiError, AppError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RoleError {
    #[error("Role not found")]
    RoleNotFound,

    #[error("Duplicate role code")]
    DuplicateRoleCode,
}

impl ApiError for RoleError {
    fn code(&self) -> u16 {
        match self {
            RoleError::RoleNotFound => 4001,
            RoleError::DuplicateRoleCode => 4002,
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<RoleError> for AppError {
    fn from(err: RoleError) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
}
