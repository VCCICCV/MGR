use sea_orm::strum;
use serde::Deserialize;

#[derive(
    Debug,
    strum::Display,
    strum::EnumString,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
)]
pub enum Profile {
    #[serde(rename = "test")]// 序列化、反序列化重命名
    #[strum(serialize = "test")]// 枚举序列化、反序列化重命名
    Test,
    #[serde(rename = "development")]
    #[strum(serialize = "development")]
    Dev,
    #[serde(rename = "production")]
    #[strum(serialize = "production")]
    Prod,
}
