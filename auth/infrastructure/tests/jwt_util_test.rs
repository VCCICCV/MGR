// use domain::model::dto::user_dto::{ LoginInfoDTO, LoginUserDTO };
// use infrastructure::utils::jwt_util::{ decode_jwt, encode_jwt };

// #[tokio::test]
// async fn test_encode_jwt() {
//     let user = LoginUserDTO {
//         email: "test@example.com".to_string(),
//         password: "test_password".to_string(),
//     };
//     let jwt = encode_jwt(user.clone()).await;
//     assert!(!jwt.is_empty());
//     println!("{}", jwt);

//     let login_info_dto = LoginInfoDTO {
//         jwt: jwt.clone(),
//     };
//     let decoded_claims = decode_jwt(&login_info_dto.jwt).await;
//     println!("{:?}", decoded_claims);
//     assert_eq!(decoded_claims.sub, user.email);
// }
// 验证逻辑
// 先验证token本身
// 再验证token与redis中的token是否相同
//
// let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("mykey".as_bytes()))?;

// let client = Client::open("redis://127.0.0.1:6379")?;
// let mut con = client.get_connection()?;
// // 使用用户邮箱作为键，Token作为值存储到Redis，并设置过期时间
// con.set_ex(email, token.clone(), (claims.exp - claims.iat) as usize)?;
