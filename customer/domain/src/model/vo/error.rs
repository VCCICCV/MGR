use axum::{ http::StatusCode, response::{ IntoResponse, Response }, Json };
use bb8_redis::redis;
// use rdkafka::error::KafkaError;
use serde::Deserialize;
use serde::Serialize;
use strum::EnumString;

pub type AppResult<T = ()> = std::result::Result<T, AppError>;
// 应用实体
pub trait AppEntity {
    const RESOURCE: ResourceType;
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resource {
    pub data: Vec<(String, String)>,
    pub resource_type: ResourceType,
}

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        self.resource_type.fmt(f)
    }
}
// 错误定义
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0} not found")] NotFoundError(Resource),
    #[error("{0} not available")] NotAvailableError(Resource),
    #[error("{0} already exists")] ResourceExistsError(Resource),
    #[error("{0}")] PermissionDeniedError(String),
    #[error("{0}")] UserNotActiveError(String),
    #[error("{0}")] InvalidSessionError(String),
    #[error("{0}")] ConflictError(String),
    #[error("{0}")] UnauthorizedError(String),
    #[error("bad request {0}")] BadRequestError(String),
    #[error("{0}")] InvalidPayloadError(String),
    #[error("{0}")] HashError(String),
    #[error(transparent)] InvalidInputError(#[from] garde::Report),
    #[error(transparent)] DatabaseError(#[from] sea_orm::error::DbErr),
    // #[error(transparent)] WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),
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
    #[error(transparent)] UnknownError(#[from] anyhow::Error),
    #[error(transparent)] Infallible(#[from] std::convert::Infallible),
    #[error(transparent)] TypeHeaderError(#[from] axum_extra::typed_header::TypedHeaderRejection),
    // #[error(transparent)] MessageError(#[from] KafkaError),
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(value: argon2::password_hash::Error) -> Self {
        AppError::HashError(value.to_string())
    }
}

impl AppError {
    pub fn response(self) -> (StatusCode, AppResponseError) {
        use AppError::*;
        let message = self.to_string();
        let (kind, code, data, status_code) = match self {
            InvalidPayloadError(_err) =>
                ("INVALID_PAYLOAD_ERROR".to_string(), None, vec![], StatusCode::BAD_REQUEST),
            BadRequestError(_err) =>
                ("BAD_REQUEST_ERROR".to_string(), None, vec![], StatusCode::BAD_REQUEST),
            NotAvailableError(resource) =>
                (format!("{resource}_NOT_AVAILABLE_ERROR"), None, vec![], StatusCode::NOT_FOUND),
            NotFoundError(resource) =>
                (
                    format!("{resource}_NOT_FOUND_ERROR"),
                    Some(resource.resource_type as i32),
                    resource.data.clone(),
                    StatusCode::NOT_FOUND,
                ),
            ResourceExistsError(resource) =>
                (
                    format!("{resource}_ALREADY_EXISTS_ERROR"),
                    Some(resource.resource_type as i32),
                    resource.data.clone(),
                    StatusCode::CONFLICT,
                ),
            AxumError(_err) =>
                ("AXUM_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            ConfigError(_err) =>
                ("CONFIG_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            AddrParseError(_err) =>
                ("ADDR_PARSE_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            IoError(err) => {
                let (status, kind, code) = match err.kind() {
                    std::io::ErrorKind::NotFound =>
                        (
                            StatusCode::NOT_FOUND,
                            format!("{}_NOT_FOUND_ERROR", ResourceType::File),
                            Some(ResourceType::File as i32),
                        ),
                    std::io::ErrorKind::PermissionDenied => {
                        (StatusCode::FORBIDDEN, "FORBIDDEN_ERROR".to_string(), None)
                    }
                    _ => (StatusCode::INTERNAL_SERVER_ERROR, "IO_ERROR".to_string(), None),
                };
                (kind, code, vec![], status)
            }
            // WebSocketError(_err) =>
            //     ("WEBSOCKET_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            ParseJsonError(_err) =>
                ("PARSE_JSON_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            StrumParseError(_err) =>
                ("STRUM_PARSE_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            HttpClientError(_err) =>
                ("HTTP_CLIENT_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            SystemTimeError(_err) =>
                ("SYSTEM_TIME_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            SpawnTaskError(_err) =>
                ("SPAWN_TASK_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            UnknownError(_err) =>
                ("UNKNOWN_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            PermissionDeniedError(_err) =>
                ("PERMISSION_DENIED_ERROR".to_string(), None, vec![], StatusCode::FORBIDDEN),
            InvalidSessionError(_err) =>
                ("INVALID_SESSION_ERROR".to_string(), None, vec![], StatusCode::BAD_REQUEST),
            ConflictError(_err) =>
                ("CONFLICT_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            UserNotActiveError(_err) =>
                ("USER_NOT_ACTIVE_ERROR".to_string(), None, vec![], StatusCode::FORBIDDEN),
            UnauthorizedError(_err) =>
                ("UNAUTHORIZED_ERROR".to_string(), None, vec![], StatusCode::UNAUTHORIZED),
            UuidError(_err) =>
                ("UUID_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            JwtError(_err) =>
                ("UNAUTHORIZED_ERROR".to_string(), None, vec![], StatusCode::UNAUTHORIZED),
            RedisError(_err) =>
                ("REDIS_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            SmtpError(_err) =>
                ("SMTP_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            LetterError(_err) =>
                ("LETTER_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            HashError(_err) =>
                ("HASH_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            ParseFloatError(_err) =>
                ("PARSE_FLOAT_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            Base64Error(_err) =>
                ("BASE64_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            InvalidInputError(err) =>
                (
                    "INVALID_INPUT_ERROR".to_string(),
                    None,
                    err
                        .iter()
                        .map(|(p, e)| (p.to_string(), e.to_string()))
                        .collect(),
                    StatusCode::BAD_REQUEST,
                ),
            DatabaseError(_err) =>
                ("DATABASE_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            Infallible(_err) =>
                ("INFALLIBLE".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            RedisError(_redis_error) =>
                ("REDIS_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            TypeHeaderError(_err) =>
                ("TYPE_HEADER_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
            // MessageError(_kafka_error) =>
            //     ("KAFKA_ERROR".to_string(), None, vec![], StatusCode::INTERNAL_SERVER_ERROR),
        };

        (status_code, AppResponseError::new(kind, message, code, data))
    }
}
// 错误响应
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, body) = self.response();
        (status_code, Json(body)).into_response()
    }
}
// 错误响应
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, utoipa::ToSchema)]
pub struct AppResponseError {
    pub kind: String,
    pub code: Option<i32>,
    pub data: Vec<(String, String)>,
    pub message: String,
}

impl AppResponseError {
    pub fn new(
        kind: impl Into<String>,
        message: impl Into<String>,
        code: Option<i32>,
        data: Vec<(String, String)>
    ) -> Self {
        Self {
            kind: kind.into(),
            message: message.into(),
            code,
            data,
        }
    }
}

#[derive(Debug, EnumString, strum::Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceType {
    #[strum(serialize = "USER")]
    User,
    #[strum(serialize = "FILE")]
    File,
    #[strum(serialize = "SESSION")]
    Session,
    #[strum(serialize = "MESSAGE")]
    Message,
}

pub fn invalid_input_error(field: &'static str, message: &'static str) -> AppError {
    let mut report = garde::Report::new();
    report.append(garde::Path::new(field), garde::Error::new(message));
    AppError::InvalidInputError(report)
}

pub trait ToAppResult {
    type Output: AppEntity;
    fn to_result(self) -> AppResult<Self::Output>;
    fn check_absent(self) -> AppResult;
    fn check_absent_data(self, data: Vec<(String, String)>) -> AppResult;
    fn to_result_data(self, data: Vec<(String, String)>) -> AppResult<Self::Output>;
}

impl<T> ToAppResult for Option<T> where T: AppEntity {
    type Output = T;
    fn to_result(self) -> AppResult<Self::Output> {
        self.ok_or_else(|| {
            AppError::NotFoundError(Resource {
                data: vec![],
                resource_type: Self::Output::RESOURCE,
            })
        })
    }
    // 转响应
    fn to_result_data(self, data: Vec<(String, String)>) -> AppResult<Self::Output> {
        self.ok_or_else(|| {
            AppError::NotFoundError(Resource {
                data,
                resource_type: Self::Output::RESOURCE,
            })
        })
    }

    fn check_absent(self) -> AppResult {
        if self.is_some() {
            Err(
                AppError::ResourceExistsError(Resource {
                    data: vec![],
                    resource_type: Self::Output::RESOURCE,
                })
            )
        } else {
            Ok(())
        }
    }

    fn check_absent_data(self, data: Vec<(String, String)>) -> AppResult {
        if self.is_some() {
            Err(
                AppError::ResourceExistsError(Resource {
                    data,
                    resource_type: Self::Output::RESOURCE,
                })
            )
        } else {
            Ok(())
        }
    }
}