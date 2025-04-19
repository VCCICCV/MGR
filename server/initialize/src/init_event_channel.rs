use shared::{ constant::SystemEvent, global };
use service::admin::{
    api_key_validate_listener,
    auth_login_listener,
    jwt_created_listener,
    sys_operation_log_listener,
};

pub async fn initialize_event_channel() {
    // 注册全局事件监听
    global::register_event_listeners(
        Box::new(|rx| Box::pin(global::jwt_created_listener(rx))),
        &[
            (
                SystemEvent::AuthLoggedInEvent.to_string(),
                Box::new(|rx| Box::pin(auth_login_listener(rx))),
            ),
            (
                SystemEvent::AuditOperationLoggedEvent.to_string(),
                Box::new(|rx| Box::pin(sys_operation_log_listener(rx))),
            ),
            (
                SystemEvent::AuthApiKeyValidatedEvent.to_string(),
                Box::new(|rx| Box::pin(api_key_validate_listener(rx))),
            ),
        ]
    ).await;
}
