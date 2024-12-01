use axum::response::IntoResponse;
use domain::model::reponse::response::{ EmptyData, Res };
use infrastructure::{ config::{ env::get_env_source, AppConfig }, constant::ENV_PREFIX };
use tracing::info;
use crate::{ cmd::shutdown::shutdown_signal, routers::setup_routers, state::AppState };

pub async fn start() -> anyhow::Result<()> {
    // 加载.env 环境配置文件，成功返回包含的值，失败返回None
    dotenvy::dotenv().ok();
    // 初始化日志
    let file_appender_guard = infrastructure::config::tracing::init()?;
    println!("file_appender_guard");
    info!("The initialization of Tracing was successful");
    // 加载AppState
    let conf = AppConfig::read(get_env_source(ENV_PREFIX))?;
    info!("The initialization of Settings was successful");
    let state = AppState::new(conf.clone()).await?;
    info!("The initialization of AppState was successful");
    // 路由以及后备处理
    let app = setup_routers(state).await.fallback(handler_404);
    // 端口绑定
    let listener = tokio::net::TcpListener::bind(conf.server.get_socket_addr()?).await?;
    // 调用 `tracing` 包的 `info!`，放在启动服务之前，因为会被move
    info!("🚀 listening on {}", &listener.local_addr()?);
    // 启动服务
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal().await).await
        .unwrap();
    // 在程序结束前释放资源，保证文件写入后释放
    drop(file_appender_guard);
    Ok(())
}

/// 404处理
async fn handler_404() -> impl IntoResponse {
    Res::<EmptyData>::with_not_found()
}
