// use anyhow::Ok;
// use common::error::InfraError;
// use domain::{model::aggregate::user::User, repositories::user_repository::UserRepository};

// use crate::{assembler::user_assembler, dto::request_command::UserRegisterCommand};

// pub struct UserUseCase<U>
// where U: UserRepository,

// {
//     user_repository: U,

// }
// impl<U> UserUseCase<U>
// where U: UserRepository,
// {
//     pub fn new(user_repository: U,user_assembler:A) -> Self {
//         Self {
//             user_repository,
//             user_assembler,
//         }
//     }
//     // 执行用例逻辑
//     pub async fn execute(&self, user_register_command: UserRegisterCommand) -> Result<(), InfraError> {
//         // Ok()
//         // 转换BO
//         let user:User =
//         unimplemented!("UserUseCase::execute")
//     }
// }

// use common::error::AppError;
// use domain::{
//     repositories::user_repository::UserRepository,
// };

// use crate::dto::request_command::UserRegisterCommand;

// pub struct UserUseCase {
//     // 动态分发，在运行时确定具体的实现
//     user_service: UserService<Box<dyn UserRepository>>,
// }
// impl UserUseCase {
//     pub fn new(user_service: UserService<Box<dyn UserRepository>>) -> Self {
//         Self {
//             user_service,
//         }
//     }
// }
// impl UserUseCase {
//     pub async fn register(&self, user: UserRegisterCommand) -> Result<(), AppError> {
//         self.user_service.create_user(user).await
//     }
// }
//     // pub async fn list_users(&self) -> Result<Vec<User>, InfraError> {
//     //     // 这里使用领域服务来获取用户列表
//     //     self.user_service.find_all_users().await
//     // }

//     // pub async fn get_user_by_id(&self, id: i32) -> Result<Option<User>, InfraError> {
//     //     self.user_service.find_user_by_id(id).await
//     // }

//     // pub async fn create_user(&self, user: RegisterUserDTO) -> Result<bool, InfraError> {
//     //     self.user_service.create_user(user).await
//     // }

//     // pub async fn update_user(&self, user: User) -> Result<bool, InfraError> {
//     //     self.user_service.update_user(user).await
//     // }
//     // pub async fn delete_user(&self, id: i32) -> Result<bool, InfraError> {
//     //     self.user_service.delete_user(id).await
//     // }
// }

use common::error::{AppError, InfraError};
use domain::{
    model::aggregate::customer::Customer,
    repositories::customer_repository::CustomerRepository,
};

use crate::dto::response_dto::ListData;
pub struct UserUseCase<U> where U: CustomerRepository {
    user_repository: U,
}
impl<U> UserUseCase<U> where U: CustomerRepository {
    pub async fn get_all(&self) -> Result<ListData<Customer>, AppError> {
        match self.user_repository.find_all().await {
            Ok(customers) => {
                // 转换为DTO
                let list_data = ListData { list: customers };
                Ok(list_data)
            }
            Err(err) => {
                // 根据不同的 InfraError 类型转换为 AppError
                match err {
                    InfraError::DatabaseError(_) =>
                        Err(AppError::OtherError("Database error".to_string())),
                    InfraError::RedisError(_) =>
                        Err(AppError::OtherError("Redis error".to_string())),
                    // 其他错误类型的转换
                    _ => Err(AppError::OtherError("Unknown error".to_string())),
                }
            }
        }
    }
}
