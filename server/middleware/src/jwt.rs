use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use axum_casbin::CasbinVals;
use headers::{ authorization::Bearer, Authorization, HeaderMapExt };
use shared::{ utils::jwt::JwtUtils, web::{ auth::User, res::Res } };

pub async fn jwt_auth_middleware(
    mut req: Request<Body>,
    next: Next,
    audience: &str
) -> impl IntoResponse {
    let token = match req.headers().typed_get::<Authorization<Bearer>>() {
        Some(auth) => auth.token().to_string(),
        None => {
            return Res::<String>
                ::new_error(
                    StatusCode::UNAUTHORIZED.as_u16(),
                    "No token provided or invalid token type"
                )
                .into_response();
        }
    };

    match JwtUtils::validate_token(&token, audience).await {
        Ok(data) => {
            let claims = data.claims;
            let user = User::from(claims);
            let vals = CasbinVals {
                subject: user.subject(),
                domain: Option::from(user.domain()),
            };
            // 将用户信息和认证信息存入请求的扩展中
            req.extensions_mut().insert(user);
            req.extensions_mut().insert(vals);
            next.run(req).await.into_response()
        }
        Err(err) => {
            Res::<String>
                ::new_error(StatusCode::UNAUTHORIZED.as_u16(), err.to_string().as_str())
                .into_response()
        }
    }
}
