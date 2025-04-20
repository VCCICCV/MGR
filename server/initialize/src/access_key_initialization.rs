use service::admin::sys_access_key_service::{SysAccessKeyService, TAccessKeyService};
use tracing::info;


pub async fn initialize_access_key() {
    let access_key_service = SysAccessKeyService;

    let _ = access_key_service.initialize_access_key().await;

    info!("Access key initialization completed successfully")
}
