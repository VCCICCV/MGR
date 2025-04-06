use axum::{extract::State, Json};
use tracing::error;

use anyhow::Result;

use crate::model::dto::response::{MessageResponse, Res};
pub async fn health_check() -> Result<Json<Res<MessageResponse>>, ()> {
    Ok(Json(Res::with_success(MessageResponse::new("Server is running"))))
}
