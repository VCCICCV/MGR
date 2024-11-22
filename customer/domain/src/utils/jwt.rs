// use chrono::{ Duration, Utc };
// use jsonwebtoken::{ decode, encode, DecodingKey, EncodingKey, Header, Validation };
// use uuid::Uuid;

// use crate::aggregate::user::User;
// // 生成jwt
// pub async fn encode_jwt(user: User) -> String {
//     let claims = TokenClaims {
//         sub: user.email,
//         iat: Utc::now().timestamp() as usize,
//         exp: (Utc::now() + Duration::minutes(1)).timestamp() as usize,
//         jti: Uuid::new_v4().to_string(),
//     };
//     encode(&Header::default(), &claims, &EncodingKey::from_secret("mykey".as_bytes())).expect(
//         "Failed to encode JWT"
//     )
// }
// pub async fn decode_jwt(token: &str) -> TokenClaims {
//     let key = "mykey";
//     let validation = Validation::default();
//     let decoded = decode::<TokenClaims>(
//         token,
//         &DecodingKey::from_secret(key.as_bytes()),
//         &validation
//     ).expect("Failed to decode JWT");
//     decoded.claims
// }
// use jsonwebtoken::{encode, EncodingKey, Header};
// use serde_json::json;
// use uuid::Uuid;

// pub struct JWTUtil {
//     secret: Vec<u8>,
//     issuer: String,
//     expiration: u64,
// }

// impl JWTUtil {
//     pub fn new() -> Self {
//         JWTUtil {
//             secret: "SecretKey039245678901232039487623456783092349288901402967890140939827".as_bytes().to_vec(),
//             issuer: "congo-mall".to_string(),
//             expiration: 86400,
//         }
//     }

//     pub fn generate_access_token(&self, username: String, email: String, user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
//         let claims = json!({
//             "username": username,
//             "account_number": email,
//             "customer_user_id": user_id
//         });
//         let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(self.secret.as_slice()))?;
//         Ok(token)
//     }
// }