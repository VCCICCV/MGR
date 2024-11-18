use serde::{ Deserialize, Serialize };
use crate::model::entity::receive_address::ReceiveAddress;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Customer {
    // uuid
    user_id: String,
    // 用户名
    username: String,
    // 邮件
    email: String,
    // 密码
    password: String,
    // 头像
    avatar: Option<String>,
    // 验证码
    verify_code: Option<String>,
    // 收货地址
    receive_address: Vec<ReceiveAddress>,
}
// 建造(者结构体，包含一个需要构建的对象
#[derive(Default)]
pub struct CustomerBuilder {
    customer: Customer,
}
impl CustomerBuilder {
    pub fn new() -> Self {
        CustomerBuilder::default()
    }
    pub fn user_id(&mut self, user_id: String) -> &mut Self {
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
        }
    }
}
// getter
impl Customer {
    pub fn user_id(&self) -> &str {
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
    pub fn verify_code(&self) -> &Option<String> {
        &self.verify_code
    }
    pub fn receive_address(&self) -> &Vec<ReceiveAddress> {
        &self.receive_address
    }
}
