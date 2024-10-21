use application::dto::response_dto::{ EmptyData, Res };
use axum::{ http::{ HeaderValue, Method }, response::IntoResponse, Router };
use infrastructure::{
    config::{ env::get_env_source, AppConfig },
    constant::ENV_PREFIX,
    logger::log,
};
use tokio::signal;
use tower_http::cors::CorsLayer;
use tracing::info;
use infrastructure::state::AppState;

use crate::routers::user_routes::setup_user_routes;

pub async fn start() -> anyhow::Result<()> {
    // 加载.env 环境配置文件，成功返回包含的值，失败返回None
    dotenvy::dotenv().ok();
    // 加载AppState
    let config = AppConfig::read(get_env_source(ENV_PREFIX))?;
    let state = AppState::new(config.clone()).await?;
    info!("The initialization of Settings was successful");
    // 初始化日志
    let guard = log::setup_logs(Some(config.tracing.get_log_level())).await;
    info!("The initialization of Tracing was successful");
    // 路由以及后备处理
    let app = setup_routes().await.fallback(handler_404).with_state(state);
    // 端口绑定
    let listener = tokio::net::TcpListener::bind(config.server.get_socket_addr()?).await.unwrap();
    // 调用 `tracing` 包的 `info!`，放在启动服务之前，因为会被move
    info!("🚀 listening on {}", &listener.local_addr().unwrap());
    // 启动服务
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal()).await
        .unwrap();
    // 在程序结束前释放资源
    drop(guard);
    Ok(())
}

/// 嵌套路由
pub async fn setup_routes() -> Router<AppState> {
    Router::new()
        .nest("/users", setup_user_routes().await)
        // .nest("/auth", setup_auth_routes().await)
        //请注意，对于某些请求类型，例如发布content-style：app/json
        //需要添加“.allow_heads（[http：：header：：CONTENT_GROUP]）”
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        )
       
}
/// 404处理
async fn handler_404() -> impl IntoResponse {
    Res::<EmptyData>::with_not_found()
}
/// 优雅关闭
async fn shutdown_signal() {
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
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
