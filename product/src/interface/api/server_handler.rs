use crate::{dto::request::{MessageResponse, Res}, error::AppResult};

/// 健康检查
#[utoipa::path(
    get,
    path = "/api/server/health_check",
    responses((status = 200, description = "check service is up", body = [MessageResponse]))
)]
pub async fn health_check() -> AppResult<Res<MessageResponse>> {
    Ok(Res::with_data(MessageResponse::new("Server is up and running".to_string())))
}