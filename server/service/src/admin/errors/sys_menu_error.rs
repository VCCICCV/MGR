use server_core::web::error::{ApiError, AppError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MenuError {
    #[error("Menu not found")]
    MenuNotFound,

    #[error("Duplicate route name")]
    DuplicateRouteName,
}

impl ApiError for MenuError {
    fn code(&self) -> u16 {
        match self {
            MenuError::MenuNotFound => 3001,
            MenuError::DuplicateRouteName => 3002,
        }
    }

    fn message(&self) -> String {
        format!("{}", self)
    }
}

impl From<MenuError> for AppError {
    fn from(err: MenuError) -> Self {
        AppError {
            code: err.code(),
            message: err.message(),
        }
    }
}
