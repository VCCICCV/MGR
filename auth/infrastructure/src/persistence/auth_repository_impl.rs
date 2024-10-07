use common::error::InfraError;
use domain::{ model::dto::user_dto::LoginUserDTO, repositories::auth_repository::AuthRepository };

use crate::utils::jwt_util::encode_jwt;
pub struct AuthServiceImpl {}
impl AuthRepository for AuthServiceImpl {
    async fn generate_jwt(&self, login_user_dto: LoginUserDTO) -> Result<String, InfraError> {
        Ok(encode_jwt(login_user_dto).await)
    }

    async fn generate_password(&self, password: &str) -> Result<String, InfraError> {
        todo!()
    }

    async fn verify_password(
        &self,
        password: &str,
        hashed_password: &str
    ) -> Result<bool, InfraError> {
        todo!()
    }

    async fn verify_token(&self, token: &str) -> Result<bool, InfraError> {
        todo!()
    }
}
// use super::user_repository_impl::UserRepositoryImpl;
// pub struct AuthServiceImpl {
//     user_repository: UserRepositoryImpl,
// }
// impl AuthServiceImpl {
//     pub fn new(user_repository: UserRepositoryImpl) -> Self {
//         AuthServiceImpl { user_repository }
//     }
// }
// impl AuthService for AuthServiceImpl {
//     async fn register_user(&self, user: &User) -> Result<String, InfraError> {
//         // 先查询是否存在
//         let created = self.user_repository.create(user.clone()).await?;
//         if created {
//             // 哈希密码
//             let hashed_password = middleware::password::hash_password(&user.password).await?;
//             let new_user = User {
//                 id: None,
//                 username: user.username.clone(),
//                 email: user.email.clone(),
//                 password: hashed_password,
//                 role: 0,
//                 create_time: Utc::now().naive_utc(),
//             };
//             // 创建用户
//             self.user_repository.create(new_user).await?;
//             // 生成JWT
//             Ok(encode_jwt(new_user).await)
//         } else {
//             Err(InfraError::UserError("User creation failed".to_string()))
//         }
//     }
//     async fn logout_user(&self) -> Result<(), String> {
//         todo!()
//     }
//     async fn login_user(&self, user: &LoginUserSchema) -> Result<String, String> {
//         // 查找用户
//         let found_user = self.user_repository.find_by_email(user.email.clone()).await?;
//         match found_user {
//             Some(u) => {
//                 // 验证密码
//                 if middleware::password::verify_password(&u, &user.password).await? {
//                     // 生成JWT
//                     self.user_repository.generate_jwt(u).await
//                 } else {
//                     Err("密码错误".to_string())
//                 }
//             }
//             None => Err("用户不存在".to_string()),
//         }
//     }
//     async fn get_user_by_token(&self, token: &str) -> Result<Option<User>, String> {
//         if middleware::jwt::verify_jwt(token).await? {
//             // 如果JWT验证通过，可以根据JWT中的信息查找用户（这里简单模拟）
//             let user = User {
//                 id: None,
//                 username: "".to_string(),
//                 email: "".to_string(),
//                 password: "".to_string(),
//                 role: 0,
//                 create_time: Utc::now().naive_utc(),
//             };
//             Ok(Some(user))
//         } else {
//             Ok(None)
//         }
//     }
//     async fn verify_token(&self, token: &str) -> Result<bool, String> {
//         infrastructure::middleware::jwt::verify_jwt(token).await
//     }
//     async fn refresh_token(&self, refresh_token: &str) -> Result<String, String> {
//         // 这里可以添加刷新令牌的逻辑，如验证刷新令牌的有效性，然后重新生成访问令牌
//         todo!()
//     }
// }
