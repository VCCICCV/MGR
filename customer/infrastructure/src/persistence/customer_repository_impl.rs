use std::sync::Arc;
use axum::async_trait;
use chrono::Utc;
use domain::model::dp::customer_id::CustomerId;
use domain::model::dto::query::{ Direction, PageParams };
use domain::model::entity::user::User;
use domain::utils::password;
use sea_orm::{
    ActiveModelTrait,
    ColumnTrait,
    DatabaseTransaction,
    EntityTrait,
    PaginatorTrait,
    QueryFilter,
    QueryOrder,
    QueryResult,
    Set,
};
use test_context::futures::FutureExt;
use tracing::info;
use domain::model::aggregate::customer::{ Customer, CustomerBuilder };

use domain::repositories::customer_repository::CustomerRepository;
use shared::error::{ AppError, AppResult };
use uuid::Uuid;
use crate::client::database::DatabaseClient;
use crate::client::redis::RedisClient;
use crate::po;
use crate::po::user::{ self, Entity };

pub struct CustomerRepositoryImpl {
    db: Arc<DatabaseClient>,
    redis: Arc<RedisClient>,
}
/// 静态分发
/// 当你在代码中调用这个结构体实现的方法（如果后续为它实现了如 CustomerRepository 等相关 trait 的方法）时，编译器在编译阶段就可以明确知道具体要调用的是 CustomerRepositoryImpl 这个类型所实现的对应方法，因为类型是确定的
/// 这种基于具体类型的、在编译时就能确定调用关系的方式就是静态分发，它通常具有更好的性能，因为编译器可以进行内联优化等操作，直接生成高效的机器码来执行对应的方法调用
// 注入数据库连接
impl CustomerRepositoryImpl {
    pub fn new(db: Arc<DatabaseClient>, redis: Arc<RedisClient>) -> Self {
        Self {
            db,
            redis,
        }
    }
}
// 这里的标记是动态派发
#[async_trait]
impl CustomerRepository for CustomerRepositoryImpl {
    async fn active(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult<()> {
        let user = po::user::ActiveModel {
            user_id: Set(*customer.user_id()),
            is2fa: Set(*customer.is2fa()),
            ..Default::default()
        };
        user.update(tx).await?;
        Ok(())
    }
    async fn find_by_user_id(
        &self,
        tx: &DatabaseTransaction,
        user_id: Uuid
    ) -> AppResult<Option<Customer>> {
        let result = po::user::Entity
            ::find()
            .filter(user::Column::UserId.eq(user_id))
            .one(tx).await?;
        // 转bo，这里使用if let进行有值判断
        if let Some(model) = result {
            let customer = CustomerBuilder::new()
                .user_id(model.user_id)
                .username(model.username)
                .email(model.email)
                .avatar(model.avatar)
                .password(model.password)
                .is2fa(model.is2fa)
                .build();
            return Ok(Some(customer));
        }
        Ok(None)
    }
    async fn find_page(
        &self,
        _param: PageParams
    ) -> AppResult<Vec<domain::model::entity::user::User>> {
        todo!()
    }
    async fn save(&self, tx: &DatabaseTransaction, customer: Customer) -> AppResult<Uuid> {
        // 转po
        let user = (po::user::ActiveModel {
            user_id: Set(Uuid::new_v4()),
            username: Set(customer.username().to_owned()),
            email: Set(customer.email().to_owned()),
            password: Set(password::hash(customer.password().to_string()).await?),
            is_deleted: Set(0),
            is2fa: Set(0),
            create_time: Set(Utc::now().naive_utc()),
            ..Default::default()
        }).insert(tx).await?;
        Ok(user.user_id)
    }

    async fn check_unique_by_username(
        &self,
        tx: &DatabaseTransaction,
        username: &str
    ) -> AppResult<bool> {
        let result = user::Entity::find().filter(user::Column::Username.eq(username)).one(tx).await;
        match result {
            Ok(Some(_)) => Ok(false),
            Ok(None) => Ok(true),
            Err(e) => Err(AppError::from(e)),
        }
    }
    async fn check_unique_by_email(
        &self,
        tx: &DatabaseTransaction,
        email: &str
    ) -> AppResult<bool> {
        let result = user::Entity::find().filter(user::Column::Email.eq(email)).one(tx).await;

        match result {
            Ok(Some(_)) => Ok(false), // 如果找到了记录，说明邮箱不唯一，返回 false
            Ok(None) => Ok(true), // 如果没找到记录，说明邮箱是唯一的，返回 true
            Err(e) => {
                return Err(AppError::from(e));
            }
        }
    }
}
// #[async_trait]
// /// self是调用该实例方法当前对象的引用
// impl CustomerRepository for CustomerRepositoryImpl {
//     async fn find_all(&self) -> AppResult<Vec<Customer>> {
//         todo!()

//     async fn find_by_email(
//         &self,
//         tx: &sea_orm::DatabaseTransaction,
//         email: &str
//     ) -> AppResult<Option<Customer>> {
//         todo!()
//     }
//     async fn save(&self, tx: &sea_orm::DatabaseTransaction, customer: Customer) -> AppResult<()> {
//         // let _user = (user::ActiveModel {
//         //     user_id: Set(customer.user_id().to_owned()),
//         //     username: Set(customer.username().to_owned()),
//         //     email: Set(customer.email().to_owned()),
//         //     password: Set(customer.password().to_owned()),
//         //     create_time: Set(Utc::now().naive_utc()),
//         //     ..Default::default()
//         // }).insert(tx);
//         // Ok(())
//         todo!()
//     }

//     async fn find_by_id(
//         &self,
//         tx: &sea_orm::DatabaseTransaction,
//         id: CustomerId
//     ) -> AppResult<Option<Customer>> {
//         todo!()
//     }

//     async fn send_email(&self, tx: &sea_orm::DatabaseTransaction, email: &str) -> AppResult<()> {
//         todo!()
//     }

//     async fn find_code_by_email(
//         &self,
//         tx: &sea_orm::DatabaseTransaction,
//         email: &str
//     ) -> AppResult<Option<String>> {
//         todo!()
//     }

//     async fn check_unique_by_username(
//         &self,
//         tx: &sea_orm::DatabaseTransaction,
//         username: &str
//     ) -> AppResult<bool> {
//         todo!()
//     }

//     async fn check_unique_by_email(
//         &self,
//         tx: &sea_orm::DatabaseTransaction,
//         email: &str
//     ) -> AppResult<bool> {
//         todo!()
//     }

//     async fn find_page(
//         &self,
//         param: PageParams
//     ) -> AppResult<Vec<domain::model::entity::user::User>> {
//         // 查询对象
//         let mut select = po::user::Entity::find();
//         // 选择分页方式
//         match param.sort_direction {
//             Some(Direction::DESC) => {
//                 select = select.order_by_desc(po::user::Column::CreateTime);
//             }
//             _ => {
//                 select = select.order_by_asc(po::user::Column::CreateTime);
//             }
//         }
//         // 查询
//         let models = select.paginate(&*self.db, param.page_size).await;
//         // 转bo
//         let users: Vec<User> = models
//             .into_iter()
//             .map(|model| User::from(model))
//             .collect();
//         Ok(users)
//     }
// }
