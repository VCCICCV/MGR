// use domain::model::{dp::claims::UserClaims, vo::error::{AppError, AppResult, Resource, ResourceType}};
// use tracing::info;
// use uuid::Uuid;
// use crate::client::redis::RedisClient;
// use super::redis::SessionKey;

// pub async fn check(redis: &RedisClient, claims: &UserClaims) -> AppResult<Uuid> {
//     let session_key = SessionKey {
//         user_id: claims.uid,
//     };
//     // 获取session_id
//     let session_id = redis.get(redis, &session_key).await?.ok_or_else(|| {
//         AppError::NotFoundError(Resource {
//             data: vec![("session_key".to_string(), claims.sid.to_string())],
//             resource_type: ResourceType::Session,
//         })
//     })?;
//     //  比较session_id
//     if claims.sid != session_id {
//         info!("Session id invalid so deleting it: {session_key:?}.");
//         // 删除session
//         crate::service::redis::del(redis, &session_key).await?;
//         return Err(AppError::InvalidSessionError("Session is Invalid".to_string()));
//     }
//     Ok(claims.uid)
// }

// pub async fn set(redis: &RedisClient, user_id: Uuid) -> AppResult<Uuid> {
//     let (key, value) = generate(user_id);
//     crate::service::redis::set(redis, (&key, &value)).await?;
//     Ok(value)
// }

// pub fn generate(user_id: Uuid) -> (SessionKey, Uuid) {
//     let session_id = Uuid::new_v4();
//     let key = SessionKey { user_id };
//     (key, session_id)
// }
