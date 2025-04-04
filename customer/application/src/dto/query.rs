use garde::Validate;
use sea_orm::strum::Display;
use serde::{ Deserialize, Serialize };
use utoipa::{ IntoParams, ToSchema };
// 
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, IntoParams)]
pub struct TokenInfoQuery {
    #[garde(length(min = 30))]
    pub token: String,
}
// 忘记密码请求
#[derive(Debug, Deserialize, ToSchema, Validate, IntoParams)]
pub struct ForgetPasswordQuery {
    #[garde(email)]
    pub email: String,
}
/// 分页参数
#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct PageParams {
    // 当前页
    pub page_num: u64,
    // 每页数量
    pub page_size: u64,
    // 排序字段
    pub sort_by: Option<String>,
    // 排序方式
    pub sort_direction: Option<Direction>,
}
// 排序方式
#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy, PartialEq, PartialOrd)]
pub enum Direction {
    DESC,
    ASC,
}
impl Direction {
    pub fn as_closure<T>(&self) -> impl Fn((T, T)) -> bool where T: Ord {
        match self {
            Direction::ASC => |(a, b)| a <= b,
            Direction::DESC => |(a, b)| a >= b,
        }
    }
}
