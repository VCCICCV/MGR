use sea_orm::strum::Display;
use serde::{ Deserialize, Serialize };

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
