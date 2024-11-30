use axum::Router;
use axum::routing::post;
use crate::{ api::customer_handler::{ active, sign_up, sign_in, sign_in_2fa }, state::AppState };
pub async fn setup_customer_routers(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/api/sign_up", post(sign_up))
        .route("/api/active", post(active))
        .route("/api/sign_in", post(sign_in))
        .route("/api/sign_in_2fa", post(sign_in_2fa))
}
