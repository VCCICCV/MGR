use axum::Router;
use axum::routing::post;
use crate::{ api::customer_handler::{ active, sign_up, sign_in, sign_in_2fa }, state::AppState };
pub async fn setup_customer_routers(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/api/customer/sign_up", post(sign_up))
        .route("/api/customer/active", post(active))
        .route("/api/customer/sign_in", post(sign_in))
        .route("/api/customer/sign_in_2fa", post(sign_in_2fa))
}
