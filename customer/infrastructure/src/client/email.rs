use crate::config::AppConfig;

use crate::client::builder::ClientBuilder;
use domain::{event::email::Email, model::reponse::error::AppResult};
use lettre::{
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport,
    AsyncTransport,
    Message,
    Tokio1Executor,
};


use tracing::info;
pub type EmailClient = AsyncSmtpTransport<Tokio1Executor>;

pub trait EmailClientExt: Clone + Send + Sync + ClientBuilder {
    fn send_email(&self, email: &Email) -> impl std::future::Future<Output = AppResult>;
}

impl ClientBuilder for EmailClient {
    fn build_from_config(config: &AppConfig) -> AppResult<Self> {
        Ok(
            AsyncSmtpTransport::<Tokio1Executor>
                ::starttls_relay(&config.email.host)?
                .credentials(
                    Credentials::new(config.email.username.clone(), config.email.password.clone())
                )
                .port(config.email.port)
                .build()
        )
    }
}

impl EmailClientExt for EmailClient {
    async fn send_email(&self, email: &Email) -> AppResult {
        let resp = self
            .send(
                Message::try_from(email)?
            ).await?;
        info!("Sent email successfully code: {:?}.", resp.code());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::constant::CONFIG;
    use super::*;

    // 测试链接是否可用
    #[tokio::test]
    async fn test_smtp_email_connection() {
        let client = EmailClient::build_from_config(&CONFIG).unwrap();
        assert!(client.test_connection().await.unwrap());
    }
    // 测试发送邮件
    #[tokio::test]
    async fn test_smtp_send_email() {
        let email = Email::new(
            CONFIG.email.username.clone(),
            CONFIG.email.username.clone(),
            "chaochen497@gmail.com".to_string(),
            "锈化动力商城验证码".to_string(),
            "验证码: 123456".to_string()
        );
        let email_client = EmailClient::build_from_config(&CONFIG).unwrap();
        email_client.send_email(&email).await.unwrap();
    }
}
