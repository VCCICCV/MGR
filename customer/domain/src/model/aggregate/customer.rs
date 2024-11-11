use serde::{ Deserialize, Serialize };
use uuid::Uuid;
use crate::model::entity::receive_address::ReceiveAddress;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Customer {
    // uuid
    pub user_id: String,
    // 用户名
    pub username: String,
    // 邮件
    pub email: String,
    // 密码
    pub password: String,
    // 头像
    pub avatar: Option<String>,
    // 验证码
    pub verify_code: Option<String>,
    // 收货地址
    pub receive_address: Vec<ReceiveAddress>,
}
impl Customer {
    // 关联
    pub fn new(
        user_id: String,
        username: String,
        email: String,
        password: String,
        avatar: Option<String>,
        verify_code: Option<String>,
        receive_address: Vec<ReceiveAddress>
    ) -> Self {
        Customer {
            user_id,
            username,
            email,
            password,
            avatar,
            receive_address,
            verify_code,
        }
    }
    // 重构
    pub fn reconstruct(
        &self,
        user_id: String,
        username: String,
        email: String,
        password: String,
        avatar: Option<String>,
        verify_code: Option<String>,
        receive_address: Vec<ReceiveAddress>
    ) -> Self {
        Customer {
            user_id,
            username,
            email,
            password,
            avatar,
            receive_address,
            verify_code,
        }
    }
    // 更新id
    pub fn update(&mut self, user_id: Option<String>) {
        if let Some(user_id) = user_id {
            self.user_id = user_id;
        }
    }
    // 初始化uuid
    pub fn init_uuid(&mut self) {
        let user_id = Uuid::new_v4();
        self.user_id = user_id.to_string();
    }
}
