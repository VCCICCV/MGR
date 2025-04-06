
pub mod server_router;

use axum::{http::{HeaderValue, Method}, Router};
use server_router::setup_server_routers;
use tower_http::cors::CorsLayer;
use crate::server::state::AppState;
/// 嵌套路由
pub async fn setup_routers(state: AppState) -> Router {
    let router = Router::new()
        //请注意，对于某些请求类型，例如发布content-style：app/json
        //需要添加“.allow_heads（[http：：header：：CONTENT_GROUP]）”
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        );
    let router = setup_server_routers(router);
    // // 添加其他路由
    // let router = setup_token_routers(router.await);

    // 添加state
    router.with_state(state)
}
