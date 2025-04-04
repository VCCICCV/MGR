// use domain::model::dto::user_dto::LoginUserDTO;
// use infrastructure::utils::password_util::{ hash_password, verify_password };
// use password_hash::{ rand_core::OsRng, SaltString };

// #[tokio::test]
// async fn test_hash_password() {
//     // 加密
//     let mut rng = OsRng;
//     let salt = SaltString::generate(&mut rng);
//     let password = "test_password";
//     let result = hash_password(password).await;
//     assert!(result.is_ok());
//     let hashed_password = result.unwrap();
//     println!("hashed_password: {}", hashed_password);
//     assert!(!hashed_password.is_empty());
//     // 验证
//     let result = verify_password(&hashed_password, password).await;
//     println!("result: {:?}", result);
//     assert!(result.is_ok());
//     assert!(result.unwrap());
// }
