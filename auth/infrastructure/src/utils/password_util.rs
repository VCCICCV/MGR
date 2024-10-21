// use argon2::{ Argon2, PasswordHash, PasswordVerifier };
// use common::error::InfraError;
// use password_hash::{ rand_core::OsRng, PasswordHasher, SaltString };

// /// hash验证不需要传salt，因为hash本身就带有salt
// // 加盐加密
// pub async fn hash_password(password: &str) -> Result<String, InfraError> {
//     let salt = SaltString::generate(&mut OsRng);
//     let argon2 = Argon2::default();
//     argon2
//         .hash_password(password.as_bytes(), &salt)
//         .map_err(|_| InfraError::OtherError("Failed to hash password".to_owned()))
//         .map(|hash| hash.to_string())
// }
// // 验证密码
// pub async fn verify_password(hash_password: &str, password: &str) -> Result<bool, InfraError> {
//     let parsed_hash = PasswordHash::new(hash_password).map_err(|_| {
//         InfraError::OtherError("Failed to parse password hash".to_string())
//     })?;
//     let argon2 = Argon2::default();
//     argon2
//         .verify_password(password.as_bytes(), &parsed_hash)
//         .map_err(|_| InfraError::OtherError("Password verification failed".to_string()))
// }
