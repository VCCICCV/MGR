use tracing::{error, info};

pub async fn initialize_config(file_path: &str) {
    match config::init_from_file(file_path).await {
        Ok(_) => { info!("Configuration initialized successfully from: {}", file_path) }
        Err(e) => {
            error!("Failed to initialize config from {}: {:?}", file_path, e);
        }
    }
}
