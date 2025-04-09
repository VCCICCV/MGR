// auth_middleware.rs
pub async fn rbac_check<B>(
    role: Role,
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, AppError> {
    let claims = req.extensions()
        .get::<UserClaims>()
        .ok_or(AppError::Unauthorized)?;
    
    if claims.role != role {
        return Err(AppError::Forbidden);
    }
    
    Ok(next.run(req).await)
}

// // 使用示例
// .route("/admin", get(admin_handler).layer(middleware::from_fn(|req, next| {
//     rbac_check(Role::Admin, req, next)
// })))