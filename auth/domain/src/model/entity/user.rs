// use common::error::DomainError;
use serde::{ Deserialize, Serialize };

use crate::model::dp::user_name::UserName;
use crate::model::dp::user_email::UserEmail;
use crate::model::dp::user_password::UserPassword;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    user_id: i64,
    username: UserName,
    email: UserEmail,
    password: UserPassword,
    avatar:String,
    // pub receive_address: Vec<ReceiveAddress>,
}
impl User {
    // 创建用户
    pub fn new(
        user_id: i64,
        username: UserName,
        email: UserEmail,
        password: UserPassword,
        avatar:String, 
        // receive_address: Vec<ReceiveAddress>
    ) -> Self {
        Self {
            user_id,
            username,
            email,
            password,
            avatar,
            // receive_address,
        }
    }
    // // 获取用户名
    // pub fn get_username(&self) -> &UserName {
    //     &self.username
    // }
    // // 获取邮箱
    // pub fn get_email(&self) -> &UserEmail {
    //     &self.email
    // }
    // // 获取收货地址
    // pub fn get_receive_address(&self) -> &Vec<ReceiveAddress> {
    //     &self.receive_address
    // }
    // // 验证码
    // pub fn checkout_valid_code(
    //     &mut self,
    //     verify_code: &str,
    //     is_prod_environment: bool
    // ) -> Result<(), DomainError> {
    //     if is_prod_environment {
    //         if verify_code.trim().is_empty() {
    //             return Err(DomainError::UserEntityValidationError("验证码不能为空".to_string()));
    //         }
    //         let trimmed_verify_code = verify_code.trim();
    //         let trimmed_self_verify_code = self.verify_code.trim();
    //         if trimmed_verify_code != trimmed_self_verify_code {
    //             return Err(DomainError::UserEntityValidationError("验证码不正确".to_string()));
    //         }
    //     }
    //     Ok(())
    // }
    // 生成accessToken
    // pub fn generate_access_token(&self) -> String {
    //     format!("accessToken-{}", self.user_id)
    // }
}
