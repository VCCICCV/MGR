use std::error::Error;
use tracing::info;

pub async fn init_xdb() -> Result<(), Box<dyn Error>> {
    tokio::task::spawn_blocking(|| {
        xdb::searcher_init(Some("../../server/static/ip2region.xdb".to_string()));
    })
    .await?;
    info!("XDB initialized successfully");
    Ok(())
}
