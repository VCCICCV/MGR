// 全局错误处理

use axum::{ response::{ IntoResponse, Response }, Json };
use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0} not found")] NotFoundError(String),
    #[error("{0} not available")] NotAvailableError(String),
    #[error("{0} already exists")] ResourceExistsError(String),
    #[error("{0}")] PermissionDeniedError(String),
    #[error("{0}")] UserNotActiveError(String),
    #[error("{0}")] InvalidSessionError(String),
    #[error("{0}")] ConflictError(String),
    #[error("{0}")] UnauthorizedError(String),
    #[error("bad request {0}")] BadRequestError(String),
    #[error("{0}")] InvalidPayloadError(String),
    #[error("{0}")] HashError(String),
    #[error(transparent)] DatabaseError(#[from] sea_orm::error::DbErr),
    #[error(transparent)] WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error(transparent)] IoError(#[from] std::io::Error),
    #[error(transparent)] UuidError(#[from] uuid::Error),
    #[error(transparent)] JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)] HttpClientError(#[from] reqwest::Error),
    #[error(transparent)] RedisError(#[from] redis::RedisError),
    #[error(transparent)] ConfigError(#[from] config::ConfigError),
    #[error(transparent)] SmtpError(#[from] lettre::transport::smtp::Error),
    #[error(transparent)] LetterError(#[from] lettre::error::Error),
    #[error(transparent)] ParseJsonError(#[from] serde_json::Error),
    #[error(transparent)] ParseFloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)] AddrParseError(#[from] std::net::AddrParseError),
    #[error(transparent)] SpawnTaskError(#[from] tokio::task::JoinError),
    #[error(transparent)] Base64Error(#[from] base64::DecodeError),
    #[error(transparent)] StrumParseError(#[from] strum::ParseError),
    #[error(transparent)] SystemTimeError(#[from] std::time::SystemTimeError),
    #[error(transparent)] AxumError(#[from] axum::Error),
    #[error(transparent)] Infallible(#[from] std::convert::Infallible),
    #[error(transparent)] TypeHeaderError(#[from] axum_extra::typed_header::TypedHeaderRejection),
    #[error(transparent)] KafkaError(#[from] rdkafka::error::KafkaError),
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(value: argon2::password_hash::Error) -> Self {
        AppError::HashError(value.to_string())
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 根据错误类型确定HTTP状态码
        let status_code = match &self {
            // 4xx 客户端错误
            AppError::NotFoundError(_) => StatusCode::NOT_FOUND,
            | AppError::NotAvailableError(_)
            | AppError::PermissionDeniedError(_)
            | AppError::UserNotActiveError(_) => StatusCode::FORBIDDEN,
            AppError::ResourceExistsError(_) => StatusCode::CONFLICT,
            AppError::UnauthorizedError(_) => StatusCode::UNAUTHORIZED,
            | AppError::BadRequestError(_)
            | AppError::InvalidPayloadError(_)
            | AppError::InvalidSessionError(_) => StatusCode::BAD_REQUEST,

            // 5xx 服务端错误
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        // 构建响应体
        let body = Json(
            serde_json::json!({
            "error": self.to_string(),
            "code": status_code.as_u16(),
        })
        );

        (status_code, body).into_response()
    }
}
