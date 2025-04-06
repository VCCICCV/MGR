use axum::{
    body::Body,
    http::{ header, HeaderValue, StatusCode },
    response::{ IntoResponse, Response },
};
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

// #[derive(Debug, Serialize, Deserialize)]
// // #[derive(Debug, Serialize, Deserialize)]
// // #[derive(Debug, Serialize, Deserialize)]
// // #[derive(Debug, Serialize, Deserialize)]
// // #[derive(Debug, Serialize, Deserialize)]
// pub struct ProfileResponse {
//   pub username: String,
//   pub email: String,
//   pub is_active: bool,
//   pub is_2fa: bool,
//   pub create_at: DateTime<Utc>,
// }
#[derive(Debug, Serialize, Deserialize)]
pub struct ForgetPasswordResponse {
    pub expire_in: u64,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expire_in: u64,
}
// impl TokenResponse {
//   pub fn new(access_token: String, refresh_token: String, expire_in: u64) -> Self {
//     Self {
//       token_type: BEARER.to_string(),
//       access_token,
//       refresh_token,
//       expire_in,
//     }
//   }
// }
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String,
}
impl MessageResponse {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterResponse {
    pub user_id: Uuid,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerStatusResponse {
    pub db: bool,
    pub redis: bool,
}
/// 数据统一响应格式
#[derive(Debug, Serialize, Default)]
pub struct Res<T> {
    pub code: u16,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> IntoResponse for Res<T> where T: Serialize + Send + Sync + 'static {
    fn into_response(self) -> Response {
        // 序列化响应体，如果序列化失败，返回默认的响应体
        let json_string = match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Failed to serialize response: {}", e);
                serde_json::json!({
                    "code": 500,
                    "data": null,
                    "msg": "Internal Server Error"
                }).to_string()
            }
        };
        // 添加响应头
        Response::builder()
            .status(StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
            .header(header::CONTENT_TYPE, HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()))
            .body(Body::from(json_string))
            .unwrap()
    }
}

impl<T: Serialize> Res<T> {
    // 成功数据
    pub fn with_success(data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            message: Some("Success".to_string()),
            data: Some(data),
        }
    }
    // 失败消息
    pub fn with_err(err: &str) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: Some(err.to_string()),
            data: None,
        }
    }
}
/// 由于没有序列化的数据，所以使用空结构体作为泛型参数
#[derive(Debug, Serialize)]
pub struct EmptyData;
impl Res<EmptyData> {
    // 404
    pub fn with_not_found() -> Self {
        Self {
            code: StatusCode::NOT_FOUND.as_u16(),
            message: Some("Not Found".to_string()),
            data: None,
        }
    }
}
