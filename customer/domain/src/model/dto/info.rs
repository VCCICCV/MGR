use serde::{ de::DeserializeOwned, Deserialize, Serialize };

use uuid::Uuid;
use std::fmt::Debug;
use core::fmt::Display;
use std::time::Duration;

use crate::constant::*;

pub trait RedisKey: Debug + Display {
    type Value: Serialize + DeserializeOwned + Debug;
    const EXPIRE_TIME: Duration;
    fn expire(&self) -> Duration {
        Self::EXPIRE_TIME
    }
}
// session定义
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

// 忘记密码
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
// 登录key
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
// 登录value
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct LoginValue {
    pub code: String,
}
