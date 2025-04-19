use server_core::web::error::{ApiError, AppError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    UserNotFound,
    #[error("Authentication failed: Wrong password")]
    WrongPassword,
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Username already exists")]
    UsernameAlreadyExists,
    #[error("Invalid user status")]
    InvalidUserStatus,
}

impl ApiError for UserError {
    fn code(&self) -> u16 {
        match self {
            UserError::UserNotFound => 1001,
            UserError::WrongPassword => 1002,
            UserError::AuthenticationFailed => 1003,
            UserError::UsernameAlreadyExists => 1004,
            UserError::InvalidUserStatus => 1005,
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<UserError> for AppError {
    fn from(err: UserError) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
}
