use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};
use domain::model::{dp::claims::UserClaims, vo::error::AppError};
use infrastructure::constant::ACCESS_TOKEN_DECODE_KEY;

use crate::state::AppState;

// 从header中提取claims
#[async_trait]
impl FromRequestParts<AppState> for UserClaims {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await?;
        let user_claims = UserClaims::decode(bearer.token(), &ACCESS_TOKEN_DECODE_KEY)?.claims;
        // 检查session是否存在
        service::session::check(&state.redis, &user_claims).await?;
        Ok(user_claims)
    }
}
