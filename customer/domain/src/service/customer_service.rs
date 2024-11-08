use shared::error::InfraError;
use crate::{model::aggregate::customer::Customer, repositories::customer_repository::CustomerRepository};

pub struct CustomerService<R> where R: CustomerRepository {
    customer_repository:R,
}

impl<R> CustomerService<R> where R: CustomerRepository {
    pub fn new(customer_repository: R) -> Self {
        Self {
            customer_repository,
        }
    }
}
impl<R> CustomerService<R> where R: CustomerRepository {
    pub async fn  get_by_email(&self, email: String) -> Result<Option<Customer>, InfraError> {
        self.customer_repository.find_by_email(email).await
    }
    pub async fn get_by_customer_id(&self,)
}
















// use common::error::InfraError;


// use crate::{model::aggregate::user::User, repositories::user_repository::UserRepository};

// pub struct UserService
// where 
// U:UserRepository,
// {
// user_repository: U,
// }
// impl<U> UserService<U>
// where
// U:UserRepository,
// {
//     pub fn new(user_repository: U) -> Self {
//         Self { user_repository }
//     }
//     pub async fn register(&self, user: User) -> Result<(), InfraError> {
//         self.user_repository.save(user).await
//     }
// }













// use crate::{model::{dto::user_dto::RegisterUserDTO, user::User}, repositories::user_repository::UserRepository};
// use common::error::InfraError;

// use common::error::InfraError;

// use crate::{ aggregate::user::User, repositories::user_repository::UserRepository };

// pub struct UserService<R> where R: UserRepository {
//     user_repository: R,
// }

// impl<R> UserService<R> where R: UserRepository {
//     pub fn new(user_repository: R) -> Self {
//         UserService {
//             user_repository,
//         }
//     }
// }
// impl<R> UserService<R> where R: UserRepository {
//     pub async fn create_user(&self, user: User) -> Result<(), InfraError> {
//         self.user_repository.save(user).await
//     }
// }
// pub struct UserService<R>
// where
//     R: UserRepository,
// {
//     repository: R,
// }

// impl<R> UserService<R>
// where
//     R: UserRepository,
// {
//     /// IOC
//     pub fn new(repository: R) -> Self {
//         Self { repository }
//     }

//     pub async fn find_all_users(&self) -> Result<Vec<User>, InfraError> {
//         self.repository.find_all().await
//     }

//     pub async fn find_user_by_id(&self, id: i32) -> Result<Option<User>, InfraError> {
//         self.repository.find_by_id(id).await
//     }

//     pub async fn create_user(&self, user: RegisterUserDTO) -> Result<bool, InfraError> {
//         self.repository.create(user).await
//     }

//     pub async fn update_user(&self, user: User) -> Result<bool, InfraError> {
//         self.repository.update(user).await
//     }

//     pub async fn delete_user(&self, id: i32) -> Result<bool, InfraError> {
//         self.repository.delete(id).await
//     }
// }
// pub struct UserService<U: UserRepository> {
//     user_repository: U,
// }

// impl<U: UserRepository> UserService<U> {
//     pub fn new(user_repository: U) -> Self {
//         UserService { user_repository }
//     }

//     pub async fn register(&self, register_user_dto: &RegisterUserDTO) -> Result<User, DbErr> {
//         // 可以添加更多的业务逻辑检查，如用户名唯一性等
//         let user = User {
//             id: None,
//             username: register_user_dto.username.clone(),
//             email: register_user_dto.email.clone(),
//             password: register_user_dto.password.clone(),
//             role: 1, // 假设默认角色为1
//             create_time: Utc::now().naive_utc(),
//             update_time: None,
//         };
//         self.user_repository.save(&user).await
//     }

//     pub async fn login(&self, login_user_dto: &LoginUserDTO) -> Result<(User, String), DbErr> {
//         if let Some(user) = self.user_repository.find_by_email(login_user_dto.email.as_str()).await? {
//             if user.password == login_user_dto.password {
//                 let iat = Utc::now().timestamp() as usize;
//                 let exp = (Utc::now() + chrono::Duration::hours(1)).timestamp() as usize;
//                 let jti = Uuid::new_v4().to_string();
//                 let claims = TokenClaims {
//                     sub: user.email.clone(),
//                     iat,
//                     exp,
//                     jti,
//                 };
//                 let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
//                 let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))?;
//                 return Ok((user, token));
//             }
//         }
//         Err(DbErr::Custom("Invalid credentials".to_owned()))
//     }

//     pub async fn logout(&self, _user: &User) -> Result<(), DbErr> {
//         // 在实际中可能需要清除一些与用户相关的会话信息，这里简单返回成功
//         Ok(())
//     }

//     pub async fn forgot_password(&self, forgot_password_request_dto: &ForgotPasswordRequestDto) -> Result<(), DbErr> {
//         if let Some(user) = self.user_repository.find_by_email(forgot_password_request_dto.email.as_str()).await? {
//             // 这里可以添加发送密码重置邮件等逻辑，暂时省略
//             Ok(())
//         } else {
//             Err(DbErr::Custom("Email not found".to_owned()))
//         }
//     }
//     pub async fn reset_password(&self, reset_password_request_dto: &ResetPasswordRequestDto, user: &User) -> Result<(), DbErr> {
//         if let Ok(claims) = decode::<TokenClaims>(&reset_password_request_dto.token, &EncodingKey::from_secret(env::var("JWT_SECRET").expect("JWT_SECRET not set").as_bytes()), &Validation::default()) {
//             self.user_repository.update_password(user, reset_password_request_dto.new_password.as_str()).await?;
//             Ok(())
//         } else {
//             Err(DbErr::Custom("Invalid token".to_owned()))
//         }
//     }
// }
// pub struct UserService<U: UserRepository, A: AuthService> {
//     user_repository: U,
//     auth_service: A,
// }

// impl<U: UserRepository, A: AuthService> UserService<U, A> {
//     pub fn new(user_repository: U) -> Self {
//         UserService { user_repository, auth_service }
//     }
//     pub fn register(&self, register_user_dto: RegisterUserDTO) -> Result<User, DomainError> {
//         // 验证用户是否已存在
//         if let Ok(Some(_)) = self.user_repository.find_by_email(register_user_dto.email.as_str()) {
//             return Err(DomainError::UserAlreadyExistsError("User already exists".to_string()));
//         }

//         let user = NewUser {
//             id: Uuid::new_v4(),
//             username: register_user_dto.username.clone(),
//             email: register_user_dto.email.clone(),
//             password: register_user_dto.password.clone(),
//             user_id: todo!(),
//         };
//         // 保存用户到数据库
//         self.user_repository.save(&user).map_err(|e| DomainError::from(e))?;
//         // 生成JWT
//         self.auth_service.generate_token(&user).map_err(|e| DomainError::from(e))
//     }
// }
