// use admin::user_router::setup_user_routes;
use axum::{ http::{ HeaderValue, Method }, Router };
use application::state::AppState;
use utoipa_swagger_ui::SwaggerUi;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
pub mod customer_router;
pub mod admin {
    pub mod user_router;
}
/// 嵌套路由
pub async fn setup_routes(state: AppState) -> Router {
    let router = Router::new()
        //请注意，对于某些请求类型，例如发布content-style：app/json
        //需要添加“.allow_heads（[http：：header：：CONTENT_GROUP]）”
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        )
        .merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", ApiDoc::openapi()));
    let router = customer_router::setup_customer_routes(router);
    // 添加其他路由
    // let router = setup_user_routes(router.await);
    router.await.with_state(state)
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
