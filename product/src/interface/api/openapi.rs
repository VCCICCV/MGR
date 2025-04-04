
use utoipa::OpenApi;

// 定义 API 文档结构体
#[derive(OpenApi)]
#[openapi(
    info(version = "v0.1.0", title = "Customer API"),
    paths(
        // server api
        // crate::api::server_handler::health_check,
    ),
    components(
        schemas(

        )
    ),
    tags(
        //  指定标签信息，后续用标签区给api分组
        (name = "crate::api::server_handler", description = "Server API endpoints"),
        (name = "crate::api::customer_handler", description = "Customer API endpoints")
    )
)]
pub struct ApiDoc;
