pub mod server;
pub mod product_router;

use axum::{ http::{ HeaderValue, Method }, Router };
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tower_http::cors::CorsLayer;

use server::setup_server_routers;

use crate::{interface::api::openapi::ApiDoc, state::AppState};
/// 嵌套路由
pub async fn setup_routers(state: AppState) -> Router {
    let router = Router::new()
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", ApiDoc::openapi()));
    let router = setup_server_routers(router);
    // 添加其他路由
    let router = product_router::setup_routers(router);
    // 添加state
    router.await.with_state(state)
}
