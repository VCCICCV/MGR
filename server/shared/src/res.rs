use axum::{ body::Body, http::HeaderValue, response::{ IntoResponse, Response } };
use reqwest::{ header, StatusCode };
use serde::Serialize;
use serde::Deserialize;
use strum::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct PageRequest {
    #[serde(default = "default_current")]
    pub current: u64,
    #[serde(default = "default_size")]
    pub size: u64,
    pub sort_by: Option<String>,
    pub direction: Option<Direction>,
}
fn default_current() -> u64 {
    1
}

fn default_size() -> u64 {
    10
}

#[derive(
    Default,
    Serialize,
    Deserialize,
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord
)]
pub enum Direction {
    #[default]
    DESC,
    ASC,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PageResponse<T> {
    pub current: u64,
    pub size: u64,
    pub total: u64,
    pub data: Vec<T>,
}

#[derive(Debug, Serialize, Default)]
pub struct PageData<T> {
    pub current: u64,
    pub size: u64,
    pub total: u64,
    pub records: Vec<T>,
}
#[derive(Debug, Serialize)]
pub struct ServiceStatusResponse {
    pub db: bool,
    pub redis: bool,
}
#[derive(Debug, Serialize)]
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
// /// 由于没有序列化的数据，所以使用空结构体作为泛型参数
// #[derive(Debug, Serialize)]
// pub struct EmptyData;
// impl Res<EmptyData> {
//     // 404
//     pub fn with_not_found() -> Self {
//         Self {
//             code: StatusCode::NOT_FOUND.as_u16(),
//             message: Some("Not Found".to_string()),
//             data: None,
//         }
//     }
// }
