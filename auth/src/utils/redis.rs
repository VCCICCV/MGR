use serde::{ de::DeserializeOwned, Deserialize, Serialize };
use tracing::info;
use uuid::Uuid;


use anyhow::{ Context, Result };
use std::{ fmt::{ Debug, Display }, time::Duration };

use crate::{client::redis::{ RedisClient, RedisClientExt }, constant::{EXPIRE_FORGET_PASS_CODE_SECS, EXPIRE_REGISTER_CODE_SECS, EXPIRE_SESSION_CODE_SECS, EXPIRE_TWO_FACTOR_CODE_SECS}};
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct RegisterKey {
    pub user_id: Uuid,
}
impl RedisKey for RegisterKey {
    type Value = String;
    const EXPIRE_TIME: Duration = EXPIRE_REGISTER_CODE_SECS;
    
}
impl Display for RegisterKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "REGISTER_KEY_{}", self.user_id)
    }
}
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct SessionKey {
    pub user_id: Uuid,
}

impl RedisKey for SessionKey {
    type Value = Uuid;
    const EXPIRE_TIME: Duration = EXPIRE_SESSION_CODE_SECS;
}

impl Display for SessionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SESSION_KEY_{}", self.user_id)
    }
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct ForgetPasswordKey {
    pub user_id: Uuid,
}

impl RedisKey for ForgetPasswordKey {
    type Value = String;
    const EXPIRE_TIME: Duration = EXPIRE_FORGET_PASS_CODE_SECS;
}

impl Display for ForgetPasswordKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FORGET_PASS_KEY_{}", self.user_id)
    }
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct LoginKey {
    pub user_id: Uuid,
}

impl RedisKey for LoginKey {
    type Value = String;
    const EXPIRE_TIME: Duration = EXPIRE_TWO_FACTOR_CODE_SECS;
}

impl Display for LoginKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TWO_FACTOR_LOGIN_KEY_{}", self.user_id)
    }
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct LoginValue {
    pub code: String,
}

pub trait RedisKey: Debug + Display {
    type Value: Serialize + DeserializeOwned + Debug;
    const EXPIRE_TIME: Duration;
    fn expire(&self) -> Duration {
        Self::EXPIRE_TIME
    }
}

pub async fn set<K>(client: &RedisClient, (key, value): (&K, &K::Value)) -> Result<()>
    where K: RedisKey
{
    info!("Set value to redis key: {key:?}, value: {value:?}");
    let value = serde_json::to_string(value).context("Failed to serialize value")?;
    client
        .set(&key.to_string(), &value, K::EXPIRE_TIME).await
        .context("Failed to set key in Redis")?;
    Ok(())
}

pub async fn get<K>(client: &RedisClient, key: &K) -> Result<Option<K::Value>> where K: RedisKey {
    info!("Get value from redis key: {key}");
    let value = client.get(&key.to_string()).await.context("Failed to get key from Redis")?;

    match value {
        Some(v) => {
            let deserialized = serde_json
                ::from_str::<K::Value>(&v)
                .context("Failed to deserialize Redis value")?;
            Ok(Some(deserialized))
        }
        None => Ok(None),
    }
}

pub async fn del(client: &RedisClient, key: &impl RedisKey) -> Result<bool> {
    info!("Delete key in Redis: {key:?}");
    client.del(&key.to_string()).await.context("Failed to delete key from Redis")
}

pub async fn get_ttl(client: &RedisClient, key: &impl RedisKey) -> Result<i64> {
    info!("Get TTL for key in Redis: {key:?}");
    client.ttl(&key.to_string()).await.context("Failed to get TTL from Redis")
}

pub async fn check_exist_key(client: &RedisClient, key: &impl RedisKey) -> Result<bool> {
    info!("Check if key exists in Redis: {key:?}");
    client.exist(&key.to_string()).await.context("Failed to check key existence in Redis")
}
