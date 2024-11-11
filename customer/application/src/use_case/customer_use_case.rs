use domain::{
    model::aggregate::customer::Customer,
    repositories::customer_repository::CustomerRepository,
};
use shared::error::AppError;
use tracing::info;

pub struct CustomerUseCase<U> where U: CustomerRepository {
    customer_repository: U,
}
impl<U> CustomerUseCase<U> where U: CustomerRepository {
    pub fn new(customer_repository: U) -> Self {
        Self {
            customer_repository,
        }
    }
}
impl<U> CustomerUseCase<U> where U: CustomerRepository {
    pub async fn sign_up(&self, customer: Customer) -> Result<(), AppError> {
        info!("sign up");
        self.customer_repository.save(customer).await.map_err(|e|AppError::OtherError(e.to_string()))
    }
    pub async fn send_email(&self, email: String) -> Result<(), AppError> {
        info!("send email");
        self.customer_repository.send_email(email).await.map_err(|e|AppError::OtherError(e.to_string()))
    }
}
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

// use shared::error::{ AppError, InfraError };
// use domain::{
//     model::aggregate::customer::Customer,
//     repositories::customer_repository::CustomerRepository,
// };
// use tracing::info;
//
// use crate::dto::{request_command::CustomerCommand, response_dto::ListData};
// pub struct CustomerUseCase<U> where U: CustomerRepository {
//     customer_repository: U,
// }
// impl<U> CustomerUseCase<U> where U: CustomerRepository {
//     pub fn new(customer_repository: U) -> Self {
//         Self {
//             customer_repository,
//         }
//     }
// }
// impl<U> CustomerUseCase<U> where U: CustomerRepository {
//     pub async fn get_all(&self) -> Result<ListData<Customer>, AppError> {
//         match self.customer_repository.find_all().await {
//             Ok(customers) => {
//                 // 转换为DTO
//                 let list_data = ListData { list: customers };
//                 Ok(list_data)
//             }
//             Err(err) => {
//                 // 根据不同的 InfraError 类型转换为 AppError
//                 match err {
//                     InfraError::DatabaseError(_) =>
//                         Err(AppError::OtherError("Database error".to_string())),
//                     InfraError::RedisError(_) =>
//                         Err(AppError::OtherError("Redis error".to_string())),
//                     // 其他错误类型的转换
//                     _ => Err(AppError::OtherError("Unknown error".to_string())),
//                 }
//             }
//         }
//     }
//     pub async fn create(&self, customer: CustomerCommand) -> Result<String, AppError> {
//         // 转换BO
//         let customer: Customer = customer.into();
//         info!("{}", &format!("{:?}", customer));
//         match self.customer_repository.save(customer.clone()).await {
//             Ok(()) => {
//                 Ok("Created successfully".to_string())
//             }
//             Err(err) => {
//                 // 根据不同的 InfraError 类型转换为 AppError
//                 match err {
//                     InfraError::DatabaseError(_) =>
//                         Err(AppError::OtherError(format!("用户: {} 已存在", customer.email))),
//                     InfraError::RedisError(_) =>
//                         Err(AppError::OtherError("Redis error".to_string())),
//                     // 其他错误类型的转换
//                     _ => Err(AppError::OtherError("Unknown error".to_string())),
//                 }
//             }
//         }
//     }
//     // pub async fn send_mail(&self, customer: Customer) -> Result<(), AppError> {
//
//     // }
//     // pub async fn update_address(&self, customer: Customer) -> Result<(), AppError> {}
// }
