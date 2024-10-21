// use std::sync::Arc;
//
// use anyhow::Ok;
// use sea_orm::{ DatabaseConnection, EntityTrait };
// use common::error::InfraError;
// use domain::repositories::user_repository::UserRepository;
// use crate::client::database::DatabaseClient;
// use crate::po::prelude::User as UserEntity;
//
// #[derive(Clone)]
// pub struct UserRepositoryImpl {
//     db: Arc<DatabaseClient>,
// }
// // 注入数据库连接
// impl UserRepositoryImpl {
//     pub fn new(db: Arc<DatabaseClient>) -> Self {
//         UserRepositoryImpl {
//             db,
//         }
//     }
// }
// impl UserRepository for UserRepositoryImpl {
//     async fn find_by_email(&self, email: String) -> Result<Option<User>, InfraError> {
//         let model = UserEntity::find()
//             .filter(<entities::user::Entity as EntityTrait>::Column::Email.eq(email))
//             .one(&self.db).await?;
//         let user = model.map(|model| User {
//             user_id: Some(model.get_user_id()),
//             username: model.username,
//             email: model.email,
//             password: model.password,
//             verify_code: "".to_string(),
//             receive_address: Vec::new(),
//         });
//
//         // 转bo
//         let user = User::try_from(user);
//         Ok(user)
//     }
//
//     async fn save(&self, user: User) -> Result<(), InfraError> {
//         todo!()
//     }
//
//     async fn update(&self, user: User) -> Result<(), InfraError> {
//         todo!()
//     }
//
//     async fn delete(&self, id: i32) -> Result<(), InfraError> {
//         todo!()
//     }
// }

// use chrono::Utc;
// use common::error::InfraError;
// use domain::{
//     model::{ dto::user_dto::RegisterUserDTO, user::User },
//     repositories::user_repository::UserRepository,
// };
// use sea_orm::{ ActiveValue::NotSet, DatabaseConnection, EntityTrait, QueryFilter, Set };
// use tokio::task;
// use tracing::info;

// use crate::{
//     database::db_connection::establish_db_connection,
//     entities::{ self, user::{ ActiveModel, Entity as UserEntity } },
//     utils::redis_util::RedisUtil,
// };
// pub struct UserRepositoryImpl {
//     pool: DatabaseConnection,
// }

// impl UserRepositoryImpl {
//     pub async fn new() -> Result<Self, InfraError> {
//         // 建立数据库连接
//         let db_pool = establish_db_connection().await?;
//         Ok(UserRepositoryImpl { pool: db_pool })
//     }
// }
// impl UserRepository for UserRepositoryImpl {
//     async fn save(&self, user: &RegisterUserDTO) -> Result<bool, InfraError> {
//         let active_model = ActiveModel {
//             username: Set(user.clone().username),
//             email: Set(user.clone().email),
//             password: Set(user.clone().password),
//             create_time: Set(Utc::now().naive_utc()),
//             update_time: NotSet,
//             ..Default::default()
//         };
//         // 执行插入操作
//         let insert_result = UserEntity::insert(active_model)
//             .exec(&self.pool)// 连接数据库
//             .await
//             .map_err(|e| InfraError::InsertError(format!("fail: {}", e)));
//         // 处理插入结果
//         match insert_result {
//             Ok(_) => {
//                 // 新用户创建成功后，将用户信息存储到Redis缓存
//                 let redis_key = format!("user_{}", user.email);
//                 let redis_util = RedisUtil::new().await.expect("InfraError::RedisError");
//                 let serialized = serde_json
//                     ::to_string(&user)
//                     .map_err(|e| {
//                         InfraError::RedisError(format!("Redis serialization error: {}", e))
//                     })?;
//                 task::spawn(async move {
//                     redis_util
//                         .set_expire(&redis_key, &serialized, 60).await
//                         .expect("InfraError::RedisError");
//                 });
//                 Ok(true)
//             }
//             Err(e) => Err(InfraError::UserError(e.to_string())),
//         }
//     }

//     async fn find_by_email(&self, email: &str) -> Result<Option<User>, InfraError> {
//         info!("find_by_email: {}", email);
//         let redis_key = format!("user_{}", email);
//         let redis_util = RedisUtil::new().await.expect("InfraError::RedisError");
//         // 先查询redis
//         if let Ok(Some(cached)) = redis_util.get(&redis_key).await {
//             let user: User = serde_json
//                 ::from_str(&cached)
//                 .map_err(|e| {
//                     InfraError::RedisError(format!("Redis deserialization error: {}", e))
//                 })?;
//             return Ok(Some(user));
//         }
//         let db = establish_db_connection().await?;
//         let model = UserEntity::find()
//             .filter(<entities::user::Entity as EntityTrait>::Column::Email.eq(email))
//             .one(&db).await?;
//         let user = model.map(|model| User {
//             user_id: Some(model.user_id),
//             username: model.username,
//             email: model.email,
//             password: model.password,
//             role: model.role,
//             create_time: model.create_time,
//             update_time: model.update_time,
//             is_deleted: todo!(),
//         });
//         // 将查询结果存储到Redis缓存中
//         let serialized = serde_json
//             ::to_string(&user)
//             .map_err(|e| InfraError::RedisError(format!("Redis serialization error: {}", e)))?;
//         // 在后台异步存储到Redis缓存中，过期时间为60秒
//         task::spawn(async move {
//             redis_util
//                 .set_expire(&redis_key, &serialized, 60).await
//                 .expect("InfraError::RedisError");
//         });
//         Ok(user)
//     }
//     // async fn find_by_username(&self, username: &str) -> Result<Option<User>, InfraError> {
//     //     todo!()
//     // }
//     // async fn find_by_email(&self, email: &str) -> Result<Option<User>, InfraError> {
//     //     todo!()
//     // }
//     // async fn update_user(&self, user: &User, new_password: &str) -> Result<(), InfraError> {
//     //     todo!()
//     // }
// }

// use crate::database::db_connection::establish_db_connection;
// use crate::entities;
// use crate::entities::prelude::User as UserEntity;
// use crate::entities::user::ActiveModel;
// use crate::entities::user::Entity;
// use crate::utils::jwt_util::encode_jwt;
// use crate::utils::redis_util::RedisUtil;
// use chrono::Utc;
// use common::error::InfraError;
// use domain::model::dto::user_dto::RegisterUserDTO;
// use domain::model::user::User;
// use domain::repositories::user_repository::UserRepository;
// use sea_orm::ActiveValue::NotSet;
// use sea_orm::ColumnTrait;
// use sea_orm::DatabaseConnection;
// use sea_orm::EntityTrait;
// use sea_orm::PaginatorTrait;
// use sea_orm::QueryFilter;
// use sea_orm::Set;
// use tokio::task;
// use tracing::info;
// pub struct UserRepositoryImpl {
// }

// impl UserRepository for UserRepositoryImpl {
//     async fn find_all(&self) -> Result<Vec<User>, InfraError> {
//         let redis_key = "all_users";
//         let redis_util = RedisUtil::new().await.expect("InfraError::RedisError");
//         if let Ok(Some(cached)) = redis_util.get(redis_key).await {
//             // 从缓存中获取到数据，反序列化
//             let users: Vec<User> = serde_json
//                 ::from_str(&cached)
//                 .map_err(|e| {
//                     InfraError::RedisError(format!("Redis deserialization error: {}", e))
//                 })?;
//             return Ok(users);
//         }
//         // 如果缓存中没有，则查询数据库
//         // let db = establish_db_connection().await?;
//         let models = UserEntity::find().all(&self.db_pool).await?;
//         let users: Vec<User> = models
//             .into_iter()
//             .map(|model| User {
//                 id: Some(model.user_id),
//                 username: model.username,
//                 email: model.email,
//                 password: model.password,
//                 role: model.role,
//                 create_time: model.create_time,
//                 update_time: model.update_time,
//             })
//             .collect();
//         // 将查询结果存储到Redis缓存中
//         let serialized = serde_json
//             ::to_string(&users)
//             .map_err(|e| InfraError::RedisError(format!("Redis serialization error: {}", e)))?;
//         // 在后台异步存储到Redis缓存中，过期时间为60秒
//         task::spawn(async move {
//             redis_util
//                 .set_expire(redis_key, &serialized, 60).await
//                 .expect("InfraError::RedisError");
//         });
//         Ok(users)
//     }
//     async fn find_by_id(&self, id: i32) -> Result<Option<User>, InfraError> {
//         info!("find_by_id: {}", id);
//         let redis_key = format!("user_{}", id);
//         let redis_util = RedisUtil::new().await.expect("InfraError::RedisError");
//         // 先查询redis
//         if let Ok(Some(cached)) = redis_util.get(&redis_key).await {
//             let user: User = serde_json
//                 ::from_str(&cached)
//                 .map_err(|e| {
//                     InfraError::RedisError(format!("Redis deserialization error: {}", e))
//                 })?;
//             return Ok(Some(user));
//         }
//         let db = establish_db_connection().await?;
//         let model = UserEntity::find_by_id(id).one(&db).await?;
//         let user = model.map(|model| User {
//             id: Some(model.user_id),
//             username: model.username,
//             email: model.email,
//             password: model.password,
//             role: model.role,
//             create_time: model.create_time,
//             update_time: model.update_time,
//         });
//         // 将查询结果存储到Redis缓存中
//         let serialized = serde_json
//             ::to_string(&user)
//             .map_err(|e| InfraError::RedisError(format!("Redis serialization error: {}", e)))?;
//         // 在后台异步存储到Redis缓存中，过期时间为60秒
//         task::spawn(async move {
//             redis_util
//                 .set_expire(&redis_key, &serialized, 60).await
//                 .expect("InfraError::RedisError");
//         });
//         Ok(user)
//     }
//     async fn find_by_email(&self, email: String) -> Result<Option<User>, InfraError> {
//         info!("find_by_email: {}", email);
//         let redis_key = format!("user_{}", email);
//         let redis_util = RedisUtil::new().await.expect("InfraError::RedisError");
//         // 先查询redis
//         if let Ok(Some(cached)) = redis_util.get(&redis_key).await {
//             let user: User = serde_json
//                 ::from_str(&cached)
//                 .map_err(|e| {
//                     InfraError::RedisError(format!("Redis deserialization error: {}", e))
//                 })?;
//             return Ok(Some(user));
//         }
//         let db = establish_db_connection().await?;
//         let model = UserEntity::find()
//             .filter(<entities::user::Entity as EntityTrait>::Column::Email.eq(email))
//             .one(&db).await?;
//         let user = model.map(|model| User {
//             id: Some(model.user_id),
//             username: model.username,
//             email: model.email,
//             password: model.password,
//             role: model.role,
//             create_time: model.create_time,
//             update_time: model.update_time,
//         });
//         // 将查询结果存储到Redis缓存中
//         let serialized = serde_json
//             ::to_string(&user)
//             .map_err(|e| InfraError::RedisError(format!("Redis serialization error: {}", e)))?;
//         // 在后台异步存储到Redis缓存中，过期时间为60秒
//         task::spawn(async move {
//             redis_util
//                 .set_expire(&redis_key, &serialized, 60).await
//                 .expect("InfraError::RedisError");
//         });
//         Ok(user)
//     }

//     async fn create(&self, user: RegisterUserDTO) -> Result<bool, InfraError> {
//         let db = establish_db_connection().await.map_err(InfraError::from)?;
//         let active_model = ActiveModel {
//             username: Set(user.clone().username),
//             email: Set(user.clone().email),
//             password: Set(user.clone().password),
//             create_time: Set(Utc::now().naive_utc()),
//             update_time: NotSet,
//             ..Default::default()
//         };

//         let insert_result = UserEntity::insert(active_model)
//             .exec(&db).await
//             .map_err(|e| InfraError::InsertError(format!("fail: {}", e)));

//         match insert_result {
//             Ok(_) => {
//                 // 新用户创建成功后，将用户信息存储到Redis缓存
//                 let redis_key = format!("user_{}", user.email);
//                 let redis_util = RedisUtil::new().await.expect("InfraError::RedisError");
//                 let serialized = serde_json
//                     ::to_string(&user)
//                     .map_err(|e| {
//                         InfraError::RedisError(format!("Redis serialization error: {}", e))
//                     })?;
//                 task::spawn(async move {
//                     redis_util
//                         .set_expire(&redis_key, &serialized, 60).await
//                         .expect("InfraError::RedisError");
//                 });
//                 Ok(true)
//             }
//             Err(e) => Err(InfraError::InsertError(format!("fail: {}", e))),
//         }
//     }

//     async fn update(&self, user: User) -> Result<bool, InfraError> {
//         let db = establish_db_connection().await.map_err(InfraError::from)?;

//         //通过emial查
//         let existing_user = UserEntity::find()
//             .filter(<Entity as EntityTrait>::Column::Email.eq(user.email.clone()))
//             .one(&db).await
//             .map_err(InfraError::from)?;
//         // 存在则更新
//         if let Some(existing_user) = existing_user {
//             let active_model = ActiveModel {
//                 user_id: Set(existing_user.user_id),
//                 username: Set(user.username),
//                 email: Set(user.email),
//                 create_time: NotSet,
//                 update_time: Set(Some(Utc::now().naive_utc())),
//                 ..Default::default()
//             };
//             // 执行更新
//             let update_result = UserEntity::update_many()
//                 .filter(<Entity as EntityTrait>::Column::Email.eq(existing_user.email))
//                 .set(active_model)
//                 .exec(&db).await
//                 .map_err(InfraError::from)?;
//             // 如果更新的行数为0，则表示用户不存在
//             if update_result.rows_affected == 0 {
//                 return Err(InfraError::UserNotFound);
//             }
//             Ok(true)
//         } else {
//             // 如果用户不存在，则返回错误
//             Err(InfraError::UserNotFound)
//         }
//     }
//     // 删除用户
//     async fn delete(&self, id: i32) -> Result<bool, InfraError> {
//         let db = establish_db_connection().await?;
//         let delete_result = UserEntity::delete_many()
//             .filter(<Entity as EntityTrait>::Column::UserId.eq(id))
//             .exec(&db).await?;
//         Ok(delete_result.rows_affected > 0)
//     }
//     // 生成jwt
//     async fn generate_jwt(&self, user: User) -> Result<String, InfraError> {
//         let token = encode_jwt(user).await;
//         Ok(token)
//     }
//     // 检查用户是否存在
//     async fn user_exists(&self, email: &str) -> Result<bool, InfraError> {
//         let db = establish_db_connection().await?;
//         let count = UserEntity::find()
//             .filter(<entities::user::Entity as EntityTrait>::Column::Email.eq(email))
//             .count(&db).await?;
//         Ok(count > 0)
//     }

//     async fn generate_refresh_jwt(&self, _user: User) -> Result<String, InfraError> {
//         todo!()
//     }
// }
