use serde::Serialize;


// 创建订单命令
#[derive(Debug, Serialize)]
pub struct CreateOrderCommand {
    // 用户id
    user_id: Uuid,
    // 订单id
    order_id: Uuid,
    // 地址id
    address_id: i32,
}
// 全部分页
#[derive(Debug, Serialize)]
pub struct ProductCategoryQuery {
    // 分类名称
    name: String,
    // 父级id
    parent_id: i32,
    // 层级
    level: i32,
    // 图标
    icon_url: String,
    // 排序
    sort: i32,
    // 跳转地址
    url: String,
}
use std::fmt::Debug;
use axum::{
    body::Body,
    http::{ header, HeaderValue, StatusCode },
    response::{ IntoResponse, Response },
};
use serde::Serialize;
use utoipa::ToSchema;
//
// 查 数据返回
pub struct ListData<T> {
    pub list: Vec<T>,
}
// 转换为ListData
impl<T> From<Vec<T>> for ListData<T> {
    fn from(items: Vec<T>) -> Self {
        Self {
            list: items,
        }
    }
}
// 消息响应
#[derive(Debug, Serialize, Default, ToSchema)]
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
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> IntoResponse for Res<T> where T: Serialize + Send + Sync + Debug + 'static {
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
    pub fn with_data(data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: Some(data),
            message: Some("Success".to_string()),
        }
    }
    // 成功无数据
    pub fn with_success() -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: None,
            message: Some("Success".to_string()),
        }
    }
    // 成功消息
    pub fn with_msg(msg: &str) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: None,
            message: Some(msg.to_string()),
        }
    }
    // 成功数据和消息
    #[allow(dead_code)]
    pub fn with_data_msg(data: T, msg: &str) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            data: Some(data),
            message: Some(msg.to_string()),
        }
    }
    // 失败消息
    pub fn with_err(err: &str) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            data: None,
            message: Some(err.to_string()),
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
            data: None,
            message: Some("Not Found".to_string()),
        }
    }
}
