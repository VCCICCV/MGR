// use tracing::info;
// use uuid::Uuid;

// use crate::client::redis::RedisClient;
// use anyhow::{ Context, Result };
// use super::{ claim::UserClaims, redis::SessionKey };

// pub async fn check(redis: &RedisClient, claims: &UserClaims) -> Result<Uuid> {
//     let session_key = SessionKey { user_id: claims.uid };
//     let session_id = crate::utils::redis
//         ::get(redis, &session_key).await?
//         .ok_or_else(|| anyhow::anyhow!("Session not found for user {}", claims.uid))?;

//     if claims.sid != session_id {
//         info!("Session id invalid, deleting: {session_key:?}");
//         crate::utils::redis
//             ::del(redis, &session_key).await
//             .context("Failed to delete invalid session")?;
//         return Err(anyhow::anyhow!("Invalid session ID"));
//     }
//     Ok(claims.uid)
// }

// pub async fn set(redis: &RedisClient, user_id: Uuid) -> Result<Uuid> {
//     let (key, value) = generate(user_id);
//     crate::utils::redis
//         ::set(redis, (&key, &value)).await
//         .context("Failed to set session in Redis")?;
//     Ok(value)
// }
// pub fn generate(user_id: Uuid) -> (SessionKey, Uuid) {
//     let session_id = Uuid::new_v4();
//     let key = SessionKey { user_id };
//     (key, session_id)
// }
