use axum::async_trait;
use domain::{
    model::{
        dto::info::SessionKey,
        reponse::error::{ AppError, AppResult, Resource, ResourceType },
    },
    utils::{claim::UserClaims, session::Session},
};
use tracing::info;
use uuid::Uuid;
use std::sync::Arc;
use crate::{ client::redis::RedisClient, constant::EXPIRE_SESSION_CODE_SECS };
use crate::client::redis::RedisClientExt;
pub struct SessionImpl {
    redis: Arc<RedisClient>,
}
impl SessionImpl {
    pub fn new(redis: Arc<RedisClient>) -> Self {
        SessionImpl { redis }
    }
}
#[async_trait]
impl Session for SessionImpl {
    //  检查session_key是否存在
    async fn check(&self, claims: &UserClaims) -> AppResult<Uuid> {
        let session_key = SessionKey {
            user_id: claims.uid,
        };
        let session_id = self.redis.get(&session_key.to_string()).await?.ok_or_else(|| {
            AppError::NotFoundError(Resource {
                resource_type: ResourceType::Session,
                data: vec![("session_key".to_string(), claims.sid.to_string())],
            })
        })?;
        //  检查session_id是否一致
        if claims.sid.to_string() != session_id {
            info!("Session id invalid so deleting it: {session_key:?}.");
            self.redis.del(&session_key.to_string()).await?;
            return Err(AppError::InvalidSessionError("Session is Invalid".to_string()));
        }
        Ok(claims.uid)
    }
    //  生成session_key和session_id
    async fn set(&self, user_id: Uuid) -> AppResult<Uuid> {
        info!("Generating session for user_id: {user_id:?}.");
        // 生成session_key和session_id
        let (key, value) = generate(user_id);
        self.redis.set(&key.to_string(), &value.to_string(), EXPIRE_SESSION_CODE_SECS).await?;
        Ok(value)
    }
}
//  生成session_key和session_id
pub fn generate(user_id: Uuid) -> (SessionKey, Uuid) {
    let session_id = Uuid::new_v4();
    let key = SessionKey { user_id };
    (key, session_id)
}
