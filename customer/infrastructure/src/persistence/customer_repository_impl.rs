use std::sync::Arc;
use std::time::Duration;
use application::dto::dto::Email;
use sea_orm::{ EntityTrait, QuerySelect, Set };
use crate::client::email::EmailClientExt;
use crate::client::redis::RedisClientExt;
use crate::constant::{ CONFIG, EMAIL, REDIS };
use crate::po::prelude::ReceiveAddress;
use crate::utils::random::generate_random_code;
use domain::model::aggregate::customer::Customer;
use domain::model::vo::customer_id::CustomerId;
use domain::repositories::customer_repository::CustomerRepository;
use shared::error::InfraError;
use crate::client::database::DatabaseClient;
use crate::po::prelude::User;
use crate::po::user::ActiveModel;

pub struct CustomerRepositoryImpl {
    db: Arc<DatabaseClient>,
}
// 注入数据库连接
impl CustomerRepositoryImpl {
    pub fn new(db: Arc<DatabaseClient>) -> Self {
        Self {
            db,
        }
    }
}
impl CustomerRepository for CustomerRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<Customer>, shared::error::InfraError> {
        todo!();
        // // 查询用户表数据
        // let users = User::find().all(&*self.db).await?;
        // // 查询收货地址表数据
        // let addresses = ReceiveAddress::find().all(&*self.db).await?;

        // let mut customers: Vec<Customer> = Vec::new();

        // for user in users {
        //     let mut user_addresses: Vec<ReceiveAddress> = addresses
        //         .iter()
        //         .filter(|address| address.user_id == user.user_id)
        //         .cloned()
        //         .collect();

        //     let customer = Customer {
        //         user_id: user.user_id,
        //         username: user.username,
        //         email: user.email,
        //         password: user.password,
        //         avatar: user.avatar,
        //         receive_address: user_addresses,
        //     };

        //     customers.push(customer);
        // }
        // Ok(customers)
    }
    async fn find_by_email(
        &self,
        email: String
    ) -> Result<Option<Customer>, shared::error::InfraError> {
        todo!();
        // let model = User::find()
        //     .filter(<User as EntityTrait>::Column::Email.eq(email))
        //     .one(&*self.db).await?;
        // // 转Bo
        // Ok(model.map(|m| Customer::from(m)))
    }

    async fn save(&self, customer: Customer) -> Result<(), shared::error::InfraError> {
        // 转 po
        let active_model = ActiveModel {
            user_id: Set(customer.user_id),
            username: Set(customer.username),
            email: Set(customer.email),
            password: Set(customer.password),
            avatar: Set(customer.avatar),
            ..Default::default()
        };
        // 插入
        let _ = User::insert(active_model).exec(&*self.db).await?;
        Ok(())
    }

    async fn find_by_id(&self, id: CustomerId) -> Result<Option<Customer>, InfraError> {
        todo!()
    }

    async fn send_email(&self, email: String) -> Result<(), InfraError> {
        // 生成验证码
        let code = generate_random_code();
        // 发送到redis
        REDIS.set(&email, &code, Duration::from_secs(60)).await?;
        // 发送到邮箱
        let email = Email::new(
            CONFIG.email.username.clone(),
            CONFIG.email.username.clone(),
            email.to_string(),
            "锈化动力商城验证码".to_string(),
            format!("您的验证码是：{}", code),
        );
        EMAIL.send_email(&email).await?;
        Ok(())
    }

    async fn verify_code_send(&self, customer: Customer) -> Result<(), InfraError> {
        todo!()
    }
}
