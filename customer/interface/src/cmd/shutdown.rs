use std::future::Future;
use tokio::signal;

/// 优雅关闭
pub async fn shutdown_signal() -> impl Future<Output = ()> {
    async {
        let ctrl_c = async {
            signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix
                ::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv().await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {
                println!("Ctrl+C signal received.");
            },
            _ = terminate => {
                println!("Terminate signal received.");
            },
            else => (),
        }
    }
}
