use server_core::web::error::{ApiError, AppError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccessKeyError {
    #[error("Access key not found")]
    AccessKeyNotFound,
}

impl ApiError for AccessKeyError {
    fn code(&self) -> u16 {
        match self {
            AccessKeyError::AccessKeyNotFound => 5001,
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<AccessKeyError> for AppError {
    fn from(err: AccessKeyError) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
}
