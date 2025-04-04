use axum::{ http::StatusCode, response::{ IntoResponse, Response }, Json };
use bb8_redis::redis;
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
    #[error("{0}")] UserNotFound(String),
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
        let (kind, code, data) = match self {
            InvalidPayloadError(_err) =>
                ("INVALID_PAYLOAD_ERROR".to_string(), StatusCode::BAD_REQUEST, vec![]),
            BadRequestError(_err) =>
                ("BAD_REQUEST_ERROR".to_string(), StatusCode::BAD_REQUEST, vec![]),
            NotAvailableError(resource) =>
                (format!("{resource}_NOT_AVAILABLE_ERROR"), StatusCode::NOT_FOUND, vec![]),
            NotFoundError(resource) =>
                (
                    format!("{resource}_NOT_FOUND_ERROR"),
                    StatusCode::NOT_FOUND,
                    resource.data.clone(),
                ),
            ResourceExistsError(resource) =>
                (
                    format!("{resource}_ALREADY_EXISTS_ERROR"),
                    StatusCode::CONFLICT,
                    resource.data.clone(),
                ),
            AxumError(_err) =>
                ("AXUM_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            ConfigError(_err) =>
                ("CONFIG_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            AddrParseError(_err) =>
                ("ADDR_PARSE_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            IoError(err) => {
                let code = match err.kind() {
                    std::io::ErrorKind::NotFound => StatusCode::NOT_FOUND,
                    std::io::ErrorKind::PermissionDenied => StatusCode::FORBIDDEN,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                (
                    match err.kind() {
                        std::io::ErrorKind::NotFound =>
                            format!("{}_NOT_FOUND_ERROR", ResourceType::File),
                        _ => "IO_ERROR".to_string(),
                    },
                    code,
                    vec![],
                )
            }
            // WebSocketError(_err) =>
            //     ("WEBSOCKET_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![],),
            ParseJsonError(_err) =>
                ("PARSE_JSON_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            StrumParseError(_err) =>
                ("STRUM_PARSE_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            HttpClientError(_err) =>
                ("HTTP_CLIENT_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            SystemTimeError(_err) =>
                ("SYSTEM_TIME_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            SpawnTaskError(_err) =>
                ("SPAWN_TASK_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            UnknownError(_err) =>
                ("UNKNOWN_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            PermissionDeniedError(_err) =>
                ("PERMISSION_DENIED_ERROR".to_string(), StatusCode::FORBIDDEN, vec![]),
            InvalidSessionError(_err) =>
                ("INVALID_SESSION_ERROR".to_string(), StatusCode::BAD_REQUEST, vec![]),
            ConflictError(_err) =>
                ("CONFLICT_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            UserNotActiveError(_err) =>
                ("USER_NOT_ACTIVE_ERROR".to_string(), StatusCode::FORBIDDEN, vec![]),
            UnauthorizedError(_err) =>
                ("UNAUTHORIZED_ERROR".to_string(), StatusCode::UNAUTHORIZED, vec![]),
            UuidError(_err) =>
                ("UUID_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            JwtError(_err) => ("UNAUTHORIZED_ERROR".to_string(), StatusCode::UNAUTHORIZED, vec![]),
            RedisError(_err) =>
                ("REDIS_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            SmtpError(_err) =>
                ("SMTP_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            LetterError(_err) =>
                ("LETTER_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            HashError(_err) =>
                ("HASH_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            ParseFloatError(_err) =>
                ("PARSE_FLOAT_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            Base64Error(_err) =>
                ("BASE64_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            InvalidInputError(err) =>
                (
                    "INVALID_INPUT_ERROR".to_string(),
                    StatusCode::BAD_REQUEST,
                    err
                        .iter()
                        .map(|(p, e)| (p.to_string(), e.to_string()))
                        .collect(),
                ),
            DatabaseError(_err) =>
                ("DATABASE_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            Infallible(_err) =>
                ("INFALLIBLE".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            TypeHeaderError(_err) =>
                ("TYPE_HEADER_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            UserNotFound(_) =>
                ("DATABASE_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![]),
            // MessageError(_kafka_error) =>
            //     ("KAFKA_ERROR".to_string(), StatusCode::INTERNAL_SERVER_ERROR, vec![],),
        };

        (code, AppResponseError::new(kind, message, Some(code.as_u16()), data))
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
    pub code: Option<u16>,
    pub data: Vec<(String, String)>,
    pub message: String,
}

impl AppResponseError {
    pub fn new(
        kind: impl Into<String>,
        message: impl Into<String>,
        code: Option<u16>,
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
