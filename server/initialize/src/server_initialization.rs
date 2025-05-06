use std::error::Error;

use config::server::ServerConfig;
use shared::global;
use tracing::info;

pub async fn get_server_address() -> Result<String, Box<dyn Error>> {
    let server_config = global::get_config::<ServerConfig>().await.unwrap();
    let addr = format!("{}:{}", server_config.host, server_config.port);
    info!("Server address configured: {}", addr);
    Ok(addr)
}
