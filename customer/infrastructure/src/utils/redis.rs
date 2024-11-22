use std::fmt::Debug;
use std::fmt::Display;
use std::time::Duration;

use crate::client::redis::RedisClientExt;
use crate::constant::*;
use domain::model::vo::error::AppResult;
use fake::Dummy;

use serde::de::DeserializeOwned;
use serde::{ Deserialize, Serialize };
use tracing::info;
use uuid::Uuid;
use crate::client::redis::RedisClient;
pub trait RedisKey: Debug + Display {
    type Value: Serialize + DeserializeOwned + Debug;
    const EXPIRE_TIME: Duration;
    fn expire(&self) -> Duration {
        Self::EXPIRE_TIME
    }
}
// session定义
#[derive(Debug, Serialize, Deserialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Clone)]
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
// 登录key
#[derive(Debug, Serialize, Deserialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Clone)]
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
// 登录value
#[derive(Debug, Serialize, Deserialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct LoginValue {
    pub code: String,
}
// redis工具
// 设置值
pub async fn set<K>(client: &RedisClient, (key, value): (&K, &K::Value)) -> AppResult<()>
    where K: RedisKey
{
    info!("Set value to redis key :{key:?} value :{value:?}");
    let value = serde_json::to_string(value)?;
    client.set(&key.to_string(), &value, K::EXPIRE_TIME).await?;
    Ok(())
}
// 获取值
pub async fn get<K>(client: &RedisClient, key: &K) -> AppResult<Option<K::Value>> where K: RedisKey {
    info!("Get value from redis key :{key}");
    Ok(
        client
            .get(&key.to_string()).await?
            .map(|v| serde_json::from_str::<K::Value>(&v))
            .transpose()?
    )
}
//  删除值
pub async fn del(client: &RedisClient, key: &impl RedisKey) -> Result<bool, redis::RedisError> {
    info!("Delete key in redis :{key:?}");
    client.del(&key.to_string()).await
}
// 获取ttl
pub async fn get_tll(client: &RedisClient, key: &impl RedisKey) -> Result<i64, redis::RedisError> {
    info!("Get ttl key in redis :{key:?}");
    client.ttl(&key.to_string()).await
}
// 判断key是否存在
pub async fn check_exist_key(redis: &RedisClient, key: &impl RedisKey) -> AppResult<bool> {
    Ok(redis.exist(&key.to_string()).await?)
}

#[cfg(test)]
mod tests {
    use fake::{ Fake, Faker };

    use super::*;

    #[tokio::test]
    async fn test_set_and_get_str_redis_service() {
        let key: SessionKey = Faker.fake();
        let value = Uuid::new_v4();
        set(&REDIS, (&key, &value)).await.unwrap();
        let actual_value = get(&REDIS, &key).await.unwrap().unwrap();
        assert_eq!(actual_value, value);
    }

    #[tokio::test]
    async fn test_delete_redis_service() {
        let key: LoginKey = Faker.fake();
        let value: String = Faker.fake();
        set(&REDIS, (&key, &value)).await.unwrap();
        let actual_value = get(&REDIS, &key).await.unwrap().unwrap();
        assert_eq!(actual_value, value);
        let actual_value = del(&REDIS, &key).await.unwrap();
        assert!(actual_value);
        let actual_value = get(&REDIS, &key).await.unwrap();
        assert!(actual_value.is_none());
    }

    #[tokio::test]
    async fn test_set_and_get_value_redis_service() {
        let key: LoginKey = Faker.fake();
        let value: String = Faker.fake();
        set(&REDIS, (&key, &value)).await.unwrap();
        let actual_value = get(&REDIS, &key).await.unwrap().unwrap();
        assert_eq!(actual_value, value);
    }
}
