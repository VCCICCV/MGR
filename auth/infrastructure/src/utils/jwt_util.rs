use chrono::{ Duration, Utc };
use application::request::dto::user_dto::{LoginUserDTO, TokenClaims};
use jsonwebtoken::{ decode, encode, DecodingKey, EncodingKey, Header, Validation };
use uuid::Uuid;
// 生成jwt
pub async fn encode_jwt(user: User) -> String {
    let claims = TokenClaims {
        sub: login_user_dto.email,
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + Duration::minutes(1)).timestamp() as usize,
        jti: Uuid::new_v4().to_string(),
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret("mykey".as_bytes())).expect(
        "Failed to encode JWT"
    )
}
pub async fn decode_jwt(token: &str) -> TokenClaims {
    let key = "mykey";
    let validation = Validation::default();
    let decoded = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(key.as_bytes()),
        &validation
    ).expect("Failed to decode JWT");
    decoded.claims
}