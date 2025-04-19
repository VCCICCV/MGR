use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct JwtConfig {
    pub jwt_secret: String,
    pub issuer: String,
    pub expire: i64,
}
