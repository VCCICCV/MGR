use axum::Router;
use axum::routing::post;
use infrastructure::state::AppState;
use crate::api::customer_handler::signup;
pub async fn setup_customer_routes() -> Router<AppState> {
    Router::new()
        // .route("/get_all", get(get_all))
        .route("/signup", post(signup))
    // .route("/signin", post(signin))
    // .route("/create", post(create_customer))
    // .route("/send_mail", post(send_mail))
    // .route("/:id", delete(delete_user))
}
