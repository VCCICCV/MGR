use std::sync::Arc;
use chrono::Utc;
use domain::model::dp::customer_id::CustomerId;
use sea_orm::{ ActiveModelTrait, Set };
use tracing::info;
use domain::model::aggregate::customer::Customer;

use domain::repositories::customer_repository::CustomerRepository;
use shared::error::AppResult;
use crate::client::database::DatabaseClient;
use crate::client::redis::RedisClient;
use crate::po::user;

pub struct CustomerRepositoryImpl {
    // db: Arc<DatabaseClient>,
    // redis: Arc<RedisClient>,
}
/// 静态分发
/// 当你在代码中调用这个结构体实现的方法（如果后续为它实现了如 CustomerRepository 等相关 trait 的方法）时，编译器在编译阶段就可以明确知道具体要调用的是 CustomerRepositoryImpl 这个类型所实现的对应方法，因为类型是确定的
/// 这种基于具体类型的、在编译时就能确定调用关系的方式就是静态分发，它通常具有更好的性能，因为编译器可以进行内联优化等操作，直接生成高效的机器码来执行对应的方法调用
// 注入数据库连接
// impl CustomerRepositoryImpl {
//     pub fn new(db: Arc<DatabaseClient>, redis: Arc<RedisClient>) -> Self {
//         Self {
//             db,
//             redis,
//         }
//     }
// }
/// self是调用该实例方法当前对象的引用
impl CustomerRepository for CustomerRepositoryImpl {
     fn find_all(&self) -> AppResult<Vec<Customer>> {
        todo!()
    }

     fn find_by_email(
        &self,
        tx: &sea_orm::DatabaseTransaction,
        email: &str
    ) -> AppResult<Option<Customer>> {
        todo!()
    }
     fn save(&self, tx: &sea_orm::DatabaseTransaction, customer: Customer) -> AppResult<String> {
        let user = (user::ActiveModel {
            user_id: Set(customer.user_id().to_owned()),
            username: Set(customer.username().to_owned()),
            email: Set(customer.email().to_owned()),
            password: Set(customer.password().to_owned()),
            create_time: Set(Utc::now().naive_utc()),
            ..Default::default()
        }).insert(tx);

        Ok(user.user_id)
    }``

     fn find_by_id(
        &self,
        tx: &sea_orm::DatabaseTransaction,
        id: CustomerId
    ) -> AppResult<Option<Customer>> {
        todo!()
    }

     fn send_email(&self, tx: &sea_orm::DatabaseTransaction, email: &str) -> AppResult<()> {
        todo!()
    }

     fn find_code_by_email(
        &self,
        tx: &sea_orm::DatabaseTransaction,
        email: &str
    ) -> AppResult<Option<String>> {
        todo!()
    }

     fn check_unique_by_username(
        &self,
        tx: &sea_orm::DatabaseTransaction,
        username: &str
    ) -> AppResult<bool> {
        todo!()
    }

     fn check_unique_by_email(
        &self,
        tx: &sea_orm::DatabaseTransaction,
        email: &str
    ) -> AppResult<bool> {
        todo!()
    }
}
