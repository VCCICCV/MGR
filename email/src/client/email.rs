use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{ AsyncSmtpTransport, Tokio1Executor };
use lettre::{ AsyncTransport, Message };
use tracing::info;

use crate::{ client::ClientBuilder, model::Email };
use anyhow::{Context, Result};
use super::AppConfig;

pub type EmailClient = AsyncSmtpTransport<Tokio1Executor>;

pub trait EmailClientExt: Clone + Send + Sync + ClientBuilder {
    fn send_email(&self, email: &Email) -> impl std::future::Future<Output = Result<()>>;
}

impl ClientBuilder for EmailClient {
    fn build_from_config(config: &AppConfig) -> Result<Self> {
        Ok(
            AsyncSmtpTransport::<Tokio1Executor>
                ::relay(&config.email.host)?
                .credentials(
                    Credentials::new(config.email.username.clone(), config.email.password.clone())
                )
                .port(config.email.port)
                .tls(Tls::Required(TlsParameters::new("587".to_string()).context("Failed to create TLS parameters")?))
                .build()
        )
    }
}

impl EmailClientExt for EmailClient {
    async fn send_email(&self, email: &Email) -> Result<()> {
        let resp = self.send(Message::try_from(email)?).await?;
        info!("Sent email successfully code: {:?}.", resp.code());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::constant::CONFIG;
    use super::*;

    #[tokio::test]
    async fn test_smtp_email_connection() {
        let client = EmailClient::build_from_config(&CONFIG).unwrap();
        assert!(client.test_connection().await.unwrap());
    }
}
