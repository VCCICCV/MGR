use axum::Router;
use axum::routing::post;
use application::state::AppState;
use crate::api::customer_handler::{active, sign_up};
pub async fn setup_customer_routes(router: Router<AppState>) -> Router<AppState> {
    router
        // .route("/send_email", post(send_email))
        .route("/api/sign_up", post(sign_up))
        .route("/api/active", post(active))
    // .route("/create", post(create_customer))
    // .route("/send_mail", post(send_mail))
    // .route("/:id", delete(delete_user))
}
