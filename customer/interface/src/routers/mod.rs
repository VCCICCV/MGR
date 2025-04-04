use axum::{ http::{ HeaderValue, Method }, Router };
use server::setup_server_routers;
use token::setup_token_routers;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tower_http::cors::CorsLayer;
use crate::{ api::openapi::ApiDoc, state::AppState };
pub mod customer_router;
pub mod server;
pub mod token;
pub mod admin {
    pub mod user_router;
}
/// 嵌套路由
pub async fn setup_routers(state: AppState) -> Router {
    let router = Router::new()
        //请注意，对于某些请求类型，例如发布content-style：app/json
        //需要添加“.allow_heads（[http：：header：：CONTENT_GROUP]）”
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", ApiDoc::openapi()));
    let router = setup_server_routers(router);
    let router = customer_router::setup_customer_routers(router);
    // 添加其他路由
    let router = setup_token_routers(router.await);

    // 添加state
    router.await.with_state(state)
}
