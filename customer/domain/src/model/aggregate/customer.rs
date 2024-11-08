use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::model::entity::receive_address::ReceiveAddress;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Customer {
    // uuid
    pub user_id: i64,
    // 用户名
    pub username: String,
    // 邮件
    pub email: String,
    // 密码
    pub password: String,
    // 头像
    pub avatar: Option<String>,
    // 收货地址
    pub receive_address:Vec<ReceiveAddress>
}