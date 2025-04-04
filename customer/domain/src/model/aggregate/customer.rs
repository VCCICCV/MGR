use serde::{ Deserialize, Serialize };

use tracing::info;
use uuid::Uuid;
use crate::model::{
    dp::role::Role,
    entity::receive_address::ReceiveAddress,
    reponse::error::{ AppError, AppResult },
};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Customer {
    id: i64,
    // uuid
    user_id: Uuid,
    // 用户名
    username: String,
    // 邮件
    email: String,
    // 密码
    password: String,
    // 头像
    avatar: Option<String>,
    // 是否删除
    is_deleted: i16,
    // 验证码
    verify_code: Option<String>,
    // 二次验证
    is2fa: i16,
    // 角色
    role: Role,
    // 收货地址
    receive_address: Vec<ReceiveAddress>,
}
// 建造者结构体，包含一个需要构建的对象
#[derive(Default)]
pub struct CustomerBuilder {
    customer: Customer,
}
impl CustomerBuilder {
    pub fn new() -> Self {
        CustomerBuilder::default()
    }
    pub fn id(&mut self, id: i64) -> &mut Self {
        self.customer.id = id;
        self
    }
    pub fn user_id(&mut self, user_id: Uuid) -> &mut Self {
        self.customer.user_id = user_id;
        self
    }

    pub fn username(&mut self, username: String) -> &mut Self {
        self.customer.username = username;
        self
    }

    pub fn email(&mut self, email: String) -> &mut Self {
        self.customer.email = email;
        self
    }

    pub fn password(&mut self, password: String) -> &mut Self {
        self.customer.password = password;
        self
    }

    pub fn avatar(&mut self, avatar: Option<String>) -> &mut Self {
        self.customer.avatar = avatar;
        self
    }

    pub fn verify_code(&mut self, verify_code: Option<String>) -> &mut Self {
        self.customer.verify_code = verify_code;
        self
    }
    pub fn is_deleted(&mut self, is_deleted: i16) -> &mut Self {
        self.customer.is_deleted = is_deleted;
        self
    }
    pub fn is2fa(&mut self, is2fa: i16) -> &mut Self {
        self.customer.is2fa = is2fa;
        self
    }
    pub fn role(&mut self, role: Role) -> &mut Self {
        self.customer.role = role;
        self
    }
    pub fn receive_address(&mut self, receive_address: Vec<ReceiveAddress>) -> &mut Self {
        self.customer.receive_address = receive_address;
        self
    }
    pub fn build(&self) -> Customer {
        Customer {
            user_id: self.customer.user_id.clone(),
            username: self.customer.username.clone(),
            email: self.customer.email.clone(),
            password: self.customer.password.clone(),
            avatar: self.customer.avatar.clone(),
            verify_code: self.customer.verify_code.clone(),
            receive_address: self.customer.receive_address.clone(),
            is2fa: self.customer.is2fa.clone(),
            is_deleted: self.customer.is_deleted.clone(),
            role: self.customer.role.clone(),
            id: self.customer.id.clone(),
        }
    }
}
// getter
impl Customer {
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn email(&self) -> &str {
        &self.email
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn avatar(&self) -> &Option<String> {
        &self.avatar
    }
    pub fn is_deleted(&self) -> &i16 {
        &self.is_deleted
    }
    pub fn verify_code(&self) -> &Option<String> {
        &self.verify_code
    }
    pub fn is2fa(&self) -> &i16 {
        &self.is2fa
    }
    pub fn role(&self) -> &Role {
        &self.role
    }
    pub fn receive_address(&self) -> &Vec<ReceiveAddress> {
        &self.receive_address
    }
}
// 充血方法
impl Customer {
    // 校验验证码
    pub fn checkout_valid_code(&self, verify_code: Option<String>) -> AppResult<()> {
        match verify_code {
            None => {
                return Err(AppError::UserNotActiveError("验证码已失效".to_string()).into());
            }
            Some(code) if code.is_empty() => {
                return Err(AppError::UserNotActiveError("验证码已失效".to_string()).into());
            }
            Some(code) => {
                info!("缓存验证码：{:?}", code);
                match &self.verify_code {
                    // 未携带验证码
                    None => {
                        return Err(
                            AppError::UserNotActiveError(
                                "未找到对应的验证码记录".to_string()
                            ).into()
                        );
                    }
                    // 携带验证码与缓存的验证码不同
                    Some(saved_code) if *saved_code != code => {
                        info!("携带验证码：{:?}", saved_code);
                        return Err(AppError::UserNotActiveError("验证码错误".to_string()).into());
                    }
                    // 相同什么都不做
                    _ => {
                        ();
                    }
                }
            }
        }
        Ok(())
    }
    // 添加收货地址
    pub fn add_receive_address(&mut self, address: ReceiveAddress) {
        self.receive_address.push(address);
    }
    // 更新用户id
    pub fn update_user_id(&mut self, user_id: Uuid) {
        self.user_id = user_id;
    }
}
