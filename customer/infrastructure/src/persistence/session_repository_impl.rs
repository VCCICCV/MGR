// pub struct RedisRepositoryImpl {
//     redis: Arc<RedisClient>,
// }

// impl RedisRepositoryImpl {
//     pub fn new(redis: Arc<RedisClient>) -> Self {
//         Self {
//             redis,
//         }
//     }
// }
// #[async_trait]
// impl RedisRepository for RedisRepositoryImpl {
//     async fn check(&self, claims: &UserClaims) -> String {
//         let session_key = SessionKey {
//             user_id: claims.uid,
//         };
//         // 从 redis 中获取 session_id
//         let session_id = self.redis.get(&session_key).await?.ok_or_else(|| {
//             AppError::NotFoundError(crate::error::Resource {
//                 details: vec![("session_key".to_string(), claims.sid.to_string())],
//                 resource_type: crate::error::ResourceType::Session,
//             })
//         })?;
//         // 检查token中的session_id是否与redis中的一致
//         if claims.sid != session_id {
//             info!("Session id invalid so deleting it: {session_key:?}.");
//             crate::service::redis::del(redis, &session_key).await?;
//             return Err(AppError::InvalidSessionError("Session is Invalid".to_string()));
//         }
//         Ok(claims.uid)
//     }
//     async fn set(&self, user_id: Uuid) -> AppResult<TokenResponse> {
//         let (key, value) = generate(user_id);
//         crate::service::redis::set(redis, (&key, &value)).await?;
//         Ok(value)
//     }
// }
// //  生成session_key和session_id
// pub fn generate(user_id: Uuid) -> (SessionKey, Uuid) {
//     let session_id = Uuid::new_v4();
//     let key = SessionKey { user_id };
//     (key, session_id)
// }
