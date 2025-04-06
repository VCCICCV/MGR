use super::hash;
use anyhow::Result;
use tracing::debug;

pub async fn hash(password: String) -> Result<String> {
    let jh = tokio::task::spawn_blocking(move || hash::argon_hash(password));
    let password = jh.await
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
        .map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?;
    Ok(password)
}

pub async fn verify(password: String, hashed_pass: String) -> Result<()> {
    let jh = tokio::task::spawn_blocking(move || hash::argon_verify(password, hashed_pass));
    match jh.await {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => {
            debug!("The password is not correct: {e}");
            anyhow::bail!("The password is not correct")
        }
        Err(e) => {
            // Handle the join error
            Err(anyhow::Error::new(e).context("Failed to verify password"))
        }
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
