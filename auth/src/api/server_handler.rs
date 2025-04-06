use axum::{extract::State, Json};
use tracing::error;
use crate::{client::redis::RedisClientExt, model::dto::response::{ MessageResponse, Res, ServerStatusResponse }, server::state::AppState};
use anyhow::Result;
pub async fn health_check() -> Result<Json<Res<MessageResponse>>, ()> {
    Ok(Json(Res::with_success(MessageResponse::new("Server is running"))))
}
pub async fn server_state(State(state): State<AppState>) -> Result<Json<Res<ServerStatusResponse>>,()> {
    let db = state.db.ping().await;
    if let Err(e) = db.as_ref() {
        error!("Database connection failed error: {e}");
    }
    let redis = state.redis.ping().await;
    if let Err(e) = redis.as_ref() {
        error!("Redis connection failed error: {e}");
    }
    let resp = ServerStatusResponse {
        db: db.is_ok(),
        redis: redis.is_ok(),
    };
    Ok(Json(Res::with_success(resp)))
}
