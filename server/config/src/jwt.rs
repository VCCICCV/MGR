use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct JwtConfig {
    // 密钥
    pub jwt_secret: String,
    // 签发标识
    pub issuer: String,
    // 有效期
    pub expire: i64,
}
