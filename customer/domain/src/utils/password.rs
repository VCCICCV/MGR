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
use super::hash;
use shared::error::{invalid_input_error, AppResult};
use tracing::debug;
// hash密码
pub async fn hash(password: String) -> AppResult<String> {
    let jh = tokio::task::spawn_blocking(move || hash::argon_hash(password));
    let password = jh.await??;
    Ok(password)
}
// 校验密码
pub async fn verify(password: String, hashed_pass: String) -> AppResult {
    let jh = tokio::task::spawn_blocking(move || hash::argon_verify(password, hashed_pass));
    if let Err(e) = jh.await? {
        debug!("The password is not correct: {e}");
        Err(invalid_input_error("password", "The password is not correct."))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use fake::{ Fake, Faker };

    use super::*;

    #[tokio::test]
    pub async fn test_password_hash() {
        let password: String = Faker.fake();
        let hash_pass = hash(password).await.unwrap();
        assert!(!hash_pass.is_empty());
    }

    #[tokio::test]
    pub async fn test_password_hash_and_then_verify_it() {
        let password: String = Faker.fake();
        let hash_pass = hash(password.clone()).await.unwrap();
        verify(password, hash_pass).await.unwrap();
    }
}
