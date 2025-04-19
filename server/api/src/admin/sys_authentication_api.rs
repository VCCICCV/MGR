use std::{net::SocketAddr, sync::Arc};

use axum::{extract::ConnectInfo, http::HeaderMap, Extension};
use axum_casbin::CasbinAxumLayer;
use axum_extra::{headers::UserAgent, TypedHeader};
use server_core::web::{
    auth::User, error::AppError, res::Res, util::ClientIp, validator::ValidatedForm, RequestId,
};
use server_service::{
    admin::{
        dto::sys_auth_dto::LoginContext, AssignPermissionDto, AssignRouteDto, AuthOutput,
        LoginInput, SysAuthService, SysAuthorizationService, TAuthService, TAuthorizationService,
        UserInfoOutput, UserRoute,
    },
    Audience,
};

pub struct SysAuthenticationApi;

impl SysAuthenticationApi {
    pub async fn login_handler(
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        headers: HeaderMap,
        TypedHeader(user_agent): TypedHeader<UserAgent>,
        Extension(request_id): Extension<RequestId>,
        Extension(service): Extension<Arc<SysAuthService>>,
        ValidatedForm(input): ValidatedForm<LoginInput>,
    ) -> Result<Res<AuthOutput>, AppError> {
        let client_ip = {
            let header_ip = ClientIp::get_real_ip(&headers);
            if header_ip == "unknown" {
                addr.ip().to_string()
            } else {
                header_ip
            }
        };

        let address = xdb::searcher::search_by_ip(client_ip.as_str())
            .unwrap_or_else(|_| "Unknown Location".to_string());

        let login_context = LoginContext {
            client_ip,
            client_port: Some(addr.port() as i32),
            address,
            user_agent: user_agent.as_str().to_string(),
            request_id: request_id.to_string(),
            audience: Audience::ManagementPlatform,
            login_type: "PC".to_string(),
            domain: "built-in".to_string(),
        };

        service
            .pwd_login(input, login_context)
            .await
            .map(Res::new_data)
    }

    pub async fn get_user_info(
        Extension(user): Extension<User>,
    ) -> Result<Res<UserInfoOutput>, AppError> {
        let user_info = UserInfoOutput {
            user_id: user.user_id(),
            user_name: user.username(),
            roles: user.subject(),
        };

        Ok(Res::new_data(user_info))
    }

    pub async fn get_user_routes(
        Extension(user): Extension<User>,
        Extension(service): Extension<Arc<SysAuthService>>,
    ) -> Result<Res<UserRoute>, AppError> {
        let routes = service
            .get_user_routes(&user.subject(), &user.domain())
            .await?;

        Ok(Res::new_data(routes))
    }

    /// 为角色分配权限
    ///
    /// 将指定的权限分配给指定域中的角色。
    pub async fn assign_permission(
        Extension(service): Extension<Arc<SysAuthorizationService>>,
        Extension(mut cache_enforcer): Extension<CasbinAxumLayer>,
        ValidatedForm(input): ValidatedForm<AssignPermissionDto>,
    ) -> Result<Res<()>, AppError> {
        let enforcer = cache_enforcer.get_enforcer();

        service
            .assign_permission(input.domain, input.role_id, input.permissions, enforcer)
            .await?;

        Ok(Res::new_data(()))
    }

    /// 为角色分配路由
    ///
    /// 将指定的路由分配给指定域中的角色。
    pub async fn assign_routes(
        Extension(service): Extension<Arc<SysAuthorizationService>>,
        ValidatedForm(input): ValidatedForm<AssignRouteDto>,
    ) -> Result<Res<()>, AppError> {
        service
            .assign_routes(input.domain, input.role_id, input.route_ids)
            .await?;

        Ok(Res::new_data(()))
    }
}
