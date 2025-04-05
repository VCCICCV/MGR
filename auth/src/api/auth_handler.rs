use axum::{extract::State, Json};
use tracing::info;
use crate::model::dto::request::RegisterRequest;
use crate::model::dto::response::{RegisterResponse, Res};
use crate::server::state::AppState;


pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<Res<RegisterResponse>>, ()> {
    info!("Register a new user request: {req:?}");
    todo!()
    // match state.auth_app_service.register(&req).await {
    //     Ok(res) => Ok(Json(Res::ok(RegisterResponse::from(res)))),
    //     Err(e) => {
    //         info!("Register a new user failed: {e:?}");
    //         Err(())
    //     }
    // }
}
