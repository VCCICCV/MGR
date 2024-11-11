use std::sync::Arc;
use sea_orm::{ EntityTrait, QuerySelect, Set };
use crate::po::prelude::ReceiveAddress;
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
        // let models = User::find().all(&*self.db).await?;
        // // 将查询结果转换为Bo
        // let customers: Vec<Customer> = models.into_iter().map(Customer::from).collect();
        // Ok(customers)
        // 同时从 User 和 ReceiveAddress 表中查询并关联数据
        let results = User::find()
            .join(sea_orm::JoinType::InnerJoin, ReceiveAddress)
            .all(&*self.db).await?;

        let mut customers: Vec<Customer> = Vec::new();

        for (user, addresses) in results.into_iter().group_by(|(user, _)| user.user_id.clone()) {
            let mut receive_addresses: Vec<ReceiveAddress> = addresses
                .into_iter()
                .map(|(_, address)| address)
                .collect();
            let customer = Customer {
                user_id: user.user_id,
                username: user.username,
                email: user.email,
                password: user.password,
                avatar: user.avatar,
                receive_address: receive_addresses,
            };
            // 将地址添加到Customer中
            customers.push(customer);
        }

        Ok(customers)
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
        todo!()
    }
}
