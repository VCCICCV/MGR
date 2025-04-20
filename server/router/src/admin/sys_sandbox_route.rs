use api::admin::sys_sandbox_api::SysSandboxApi;
use axum::{routing::get, Router};

pub struct SysSandboxRouter;

impl SysSandboxRouter {
    const BASE_PATH: &str = "/sandbox";
    pub async fn init_simple_sandbox_router() -> Router {
        let router =
            Router::new().route("/simple-api-key", get(SysSandboxApi::test_simple_api_key));

        Router::new().nest(Self::BASE_PATH, router)
    }

    pub async fn init_complex_sandbox_router() -> Router {
        let router =
            Router::new().route("/complex-api-key", get(SysSandboxApi::test_complex_api_key));
        Router::new().nest(Self::BASE_PATH, router)
    }
}
