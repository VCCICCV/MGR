// use tracing::info;
// use uuid::Uuid;

// use crate::{
//     model::{ dto::dto::{ RedisKey, SessionKey }, vo::error::AppResult },
//     repositories::redis_repository::RedisRepository,
// };
// use std::sync::Arc;
// pub struct SessionService {
//     redis_repository: Arc<dyn RedisRepository>,
// }
// impl SessionService {
//     pub fn new(redis_repository: Arc<dyn RedisRepository>) -> Self {
//         SessionService { redis_repository }
//     }
// }
// // 这里使用类型约束，让函数可以接受任何实现了RedisKey的类型
// impl SessionService {
//     pub async fn get<K>(&self, key: &K) -> AppResult<Option<K::Value>> where K: RedisKey {
//         info!("Get value from redis key :{key}");
//         Ok(
//             self.redis_repository
//                 .get(&key.to_string()).await?
//                 .map(|v| serde_json::from_str::<K::Value>(&v))
//                 .transpose()?
//         )
//     }
//     pub async fn set<K>(&self, key: &K, value: K::Value) -> AppResult<()> where K: RedisKey {
//         let value = serde_json::to_string(&value)?;
//         self.redis_repository.set(&key.to_string(), &value,K::EXPIRE_TIME).await
//     }
//     pub async fn del<K>(&self, key: &K) -> AppResult<bool> where K: RedisKey {
//         info!("Delete key in redis :{key:?}");
//         self.redis_repository.del(&key.to_string()).await
//     }
//     pub async fn get_ttl<K>(&self, key: &K) -> AppResult<i64> where K: RedisKey {
//         info!("Get ttl key in redis :{key:?}");
//         self.redis_repository.get_ttl(&key.to_string()).await
//     }
//     pub async fn check_exist_key<K>(&self, key: &K) -> AppResult<bool> where K: RedisKey {
//         self.redis_repository.check_exist_key(&key.to_string()).await
//     }
// }
// // 生成session_key
// pub fn generate(user_id: Uuid) -> (SessionKey, Uuid) {
//     let session_id = Uuid::new_v4();
//     let key = SessionKey { user_id };
//     (key, session_id)
// }
