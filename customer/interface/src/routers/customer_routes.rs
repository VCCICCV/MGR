use axum::Router;
use axum::routing::post;
use infrastructure::state::AppState;
use crate::api::customer_handler::{send_email, sign_up};
pub async fn setup_customer_routes() -> Router<AppState> {
    Router::new()
        .route("/send_email", post(send_email))
        .route("/sign_up", post(sign_up))
    // .route("/signin", post(signin))
    // .route("/create", post(create_customer))
    // .route("/send_mail", post(send_mail))
    // .route("/:id", delete(delete_user))
}
