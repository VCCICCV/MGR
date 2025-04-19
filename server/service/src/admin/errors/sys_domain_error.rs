use server_core::web::error::{ApiError, AppError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Domain not found")]
    DomainNotFound,
    #[error("Domain with this code already exists")]
    DuplicateCode,
    #[error("Domain with this name already exists")]
    DuplicateName,
    #[error("Cannot modify or delete built-in domain")]
    BuiltInDomain,
}

impl ApiError for DomainError {
    fn code(&self) -> u16 {
        match self {
            DomainError::DomainNotFound => 2001,
            DomainError::DuplicateCode => 2002,
            DomainError::DuplicateName => 2003,
            DomainError::BuiltInDomain => 2004,
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
}
