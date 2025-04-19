use tracing::info;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    // 加载环境变量
    dotenvy::dotenv().ok();
    
    // 初始化日志
    let file_appender_guard = initialize::init_tracing::init();
    info!("The initialization of Tracing was successful");
    // 加载配置文件
    initialize::init_config::init_config().await;
    // 路由
    
    // let conf = initialize::init_state::init_config().await;
    // let conf = AppConfig::read(get_env_source(ENV_PREFIX))?;
    // info!("The initialization of Settings was successful");
    // let state = AppState::new(conf.clone()).await?;
    // info!("The initialization of AppState was successful");
    // // 路由以及后备处理
    // let app = setup_routers(state).fallback(handler_404);
    // // 端口绑定
    // let listener = tokio::net::TcpListener::bind(conf.server.get_socket_addr()?).await?;
    // // 调用 `tracing` 包的 `info!`，放在启动服务之前，因为会被move
    // info!("🚀 listening on {}", &listener.local_addr()?);
    // // 启动服务
    // axum::serve(listener, app.into_make_service())
    //     .with_graceful_shutdown(shutdown_signal().await).await
    //     .unwrap();
    // // 在程序结束前释放资源，保证文件写入后释放
    // drop(file_appender_guard);
    // Ok(())
}
