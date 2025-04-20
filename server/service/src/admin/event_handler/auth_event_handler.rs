use shared::web::error::AppError;

use crate::{admin::event::{access_token_event::AccessTokenEvent, login_log_event::LoginLogEvent}, helper::db_helper};
pub struct AuthEvent {
    pub user_id: String,
    pub username: String,
    pub domain: String,
    pub access_token: String,
    pub refresh_token: String,
    pub client_ip: String,
    pub client_port: Option<i32>,
    pub address: String,
    pub user_agent: String,
    pub request_id: String,
    pub login_type: String,
}

pub struct AuthEventHandler;

impl AuthEventHandler {
    pub async fn handle_login(event: AuthEvent) -> Result<(), AppError> {
        let db = db_helper::get_db_connection().await?;

        // 处理登录日志
        let login_log_event = LoginLogEvent {
            user_id: event.user_id.clone(),
            username: event.username.clone(),
            domain: event.domain.clone(),
            ip: event.client_ip.clone(),
            port: event.client_port,
            address: event.address.clone(),
            user_agent: event.user_agent.clone(),
            request_id: event.request_id.clone(),
            login_type: event.login_type.clone(),
        };

        login_log_event.handle(&db).await?;

        // 处理访问令牌
        let access_token_event = AccessTokenEvent {
            access_token: event.access_token,
            refresh_token: event.refresh_token,
            user_id: event.user_id,
            username: event.username,
            domain: event.domain,
            ip: event.client_ip,
            port: event.client_port,
            address: event.address.clone(),
            user_agent: event.user_agent,
            request_id: event.request_id,
            login_type: event.login_type,
        };

        access_token_event.handle(&db).await?;

        Ok(())
    }
}
