use application::dto::response_dto::{ EmptyData, Res };
use axum::{ http::{ HeaderValue, Method }, response::IntoResponse, routing::get, Router };
use infrastructure::{
    config::{ env::get_env_source, AppConfig },
    constant::ENV_PREFIX,
    logger::log,
};
use tower_http::cors::CorsLayer;
use tracing::info;
use infrastructure::state::AppState;
use utoipa::OpenApi;
use crate::{ cmd::shutdown::shutdown_signal, routers::customer_routes::setup_customer_routes };
use utoipa_swagger_ui::SwaggerUi;
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
    let listener = tokio::net::TcpListener::bind(config.server.get_socket_addr()?).await?;
    // 调用 `tracing` 包的 `info!`，放在启动服务之前，因为会被move
    info!("🚀 listening on {}", &listener.local_addr()?);
    // 启动服务
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal().await).await
        .unwrap();
    // 在程序结束前释放资源
    drop(guard);
    Ok(())
}
/// 嵌套路由
pub async fn setup_routes() -> Router<AppState> {
    let doc = ApiDoc::openapi();
    Router::new()
        .nest("/api/customers", setup_customer_routes().await)
        // .nest("/api//auth", setup_auth_routes().await)
        //请注意，对于某些请求类型，例如发布content-style：app/json
        //需要添加“.allow_heads（[http：：header：：CONTENT_GROUP]）”
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", doc))
}
/// 404处理
async fn handler_404() -> impl IntoResponse {
    Res::<EmptyData>::with_not_found()
}
// 定义 API 文档结构体
#[derive(OpenApi)]
#[openapi(
    tags(
        //  指定标签信息，后续用标签区给api分组
        (name = "Customer.API", description = "Customer API endpoints")
    )
)]
struct ApiDoc;
