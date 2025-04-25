use std::sync::Arc;

use axum::{ body::Body, http::StatusCode, response::IntoResponse, Extension, Router };
use axum_casbin::CasbinAxumLayer;
use chrono::Local;
use config::config::Config;
use http::Request;

use middleware::jwt_middleware::jwt_auth_middleware;
use model::entities::sys_endpoint;
use router::admin::{
    sys_access_key_route::SysAccessKeyRouter,
    sys_authentication_route::SysAuthenticationRouter,
    sys_domain_route::SysDomainRouter,
    sys_endpoint_route::SysEndpointRouter,
    sys_login_log_route::SysLoginLogRouter,
    sys_menu_route::SysMenuRouter,
    sys_operation_log_route::SysOperationLogRouter,
    sys_organization_route::SysOrganizationRouter,
    sys_role_route::SysRoleRouter,
    sys_sandbox_route::SysSandboxRouter,
    sys_user_route::SysUserRouter,
};
use service::admin::{
    sys_access_key_service::SysAccessKeyService,
    sys_auth_service::SysAuthService,
    sys_authorization_service::SysAuthorizationService,
    sys_domain_service::SysDomainService,
    sys_endpoint_service::{ SysEndpointService, TEndpointService },
    sys_login_log_service::SysLoginLogService,
    sys_menu_service::SysMenuService,
    sys_operation_log_service::SysOperationLogService,
    sys_organization_service::SysOrganizationService,
    sys_role_service::SysRoleService,
    sys_user_service::SysUserService,
};
use shared::{
    constant::Audience,
    global::{ clear_routes, get_collected_routes, get_config },
    utils::{
        self,
        api_key_middleware,
        protect_route,
        ApiKeySource,
        ApiKeyValidation,
        ComplexApiKeyConfig,
        SimpleApiKeyConfig,
        ValidatorType,
    },
    web::request_id::{ RequestId, RequestIdLayer },
};
use tower_http::trace::TraceLayer;
use tracing::{ error, info, info_span };

use crate::casbin_initialization::initialize_casbin;

#[derive(Clone)]
pub enum Services<T: Send + Sync + 'static> {
    None(std::marker::PhantomData<T>),
    Single(Arc<T>),
}

async fn apply_layers<T: Send + Sync + 'static>(
    router: Router,
    services: Services<T>,
    need_casbin: bool,
    need_auth: bool,
    api_validation: Option<ApiKeyValidation>,
    casbin: Option<CasbinAxumLayer>,
    audience: Audience
) -> Router {
    //  添加服务扩展层
    let mut router = match services {
        Services::None(_) => router,
        Services::Single(service) => router.layer(Extension(service)),
    };
    // 添加请求追踪层
    router = router
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".into());
                info_span!(
                    "[MGR >>>>>> request",
                    id = %request_id,
                    method = %request.method(),
                    uri = %request.uri(),
                )
            })
        )
        .layer(RequestIdLayer);

    // 需要鉴权
    if need_casbin {
        if let Some(casbin) = casbin {
            router = router.layer(Extension(casbin.clone())).layer(casbin);
        }
    }

    if let Some(validation) = api_validation {
        router = router.layer(
            axum::middleware::from_fn(move |req, next| {
                api_key_middleware(validation.clone(), req, next)
            })
        );
    }

    // 需要jwt验证
    if need_auth {
        router = router.layer(
            axum::middleware::from_fn(move |req, next| {
                jwt_auth_middleware(req, next, audience.as_str())
            })
        );
    }

    router
}
// 初始化admin路由
pub async fn initialize_admin_router() -> Router {
    // 清空旧路由
    clear_routes().await;
    info!("Initializing admin router");

    // 获取配置
    let app_config = get_config::<Config>().await.unwrap();
    // 初始化casbin
    let casbin_layer = initialize_casbin(
        "../static/rbac_model.conf",
        app_config.database.url.as_str()
    ).await.unwrap();

    // 初始化验证器
    // 根据是否配置了 Redis 来选择 nonce 存储实现
    let nonce_store_factory = if
        let Some(_) = crate::redis_initialization::get_primary_redis().await
    {
        // 如果 Redis 可用，使用 Redis 作为 nonce 存储
        info!("Using Redis for nonce storage");
        utils::redis_nonce_store::create_redis_nonce_store_factory("api_key")
    } else {
        // 否则使用内存存储
        info!("Using memory for nonce storage");
        utils::memory_nonce_store::create_memory_nonce_store_factory()
    };

    // 初始化验证器
    utils::init_validators_with_nonce_store(None, nonce_store_factory.clone()).await;

    let simple_validation = {
        let validator = utils::get_simple_validator().await;
        utils::add_key(ValidatorType::Simple, "test-api-key", None).await;
        ApiKeyValidation::Simple(validator, SimpleApiKeyConfig {
            source: ApiKeySource::Header,
            key_name: "x-api-key".to_string(),
        })
    };

    let complex_validation = {
        let validator = utils::get_complex_validator().await;
        utils::add_key(ValidatorType::Complex, "test-access-key", Some("test-secret-key")).await;
        ApiKeyValidation::Complex(validator, ComplexApiKeyConfig {
            key_name: "AccessKeyId".to_string(),
            timestamp_name: "t".to_string(),
            nonce_name: "n".to_string(),
            signature_name: "sign".to_string(),
        })
    };

    // 保护路由
    protect_route("/sandbox/simple-api-key");
    protect_route("/sandbox/complex-api-key");

    let audience = Audience::ManagementPlatform;
    let casbin = Some(casbin_layer);
    // 合并子路由
    let mut app = Router::new();

    macro_rules! merge_router {
        // 无服务路由
        ($router:expr, None, $need_casbin:expr, $need_auth:expr, $api_validation:expr) => {
            app = app.merge(
                apply_layers(
                    $router,
                    Services::None(std::marker::PhantomData::<()>),
                    $need_casbin,
                    $need_auth,
                    $api_validation,
                    casbin.clone(),
                    audience,
                )
                .await,
            );
        };
        // 带服务路由
        ($router:expr, $service:expr, $need_casbin:expr, $need_auth:expr, $api_validation:expr) => {
            app = app.merge(
                apply_layers(
                    $router,
                    Services::Single(Arc::new($service)),
                    $need_casbin,
                    $need_auth,
                    $api_validation,
                    casbin.clone(),
                    audience,
                )
                .await,
            );
        };
    }

    // 认证路由
    merge_router!(
        SysAuthenticationRouter::init_authentication_router().await,
        SysAuthService,
        false,
        false,
        None
    );

    // 注入服务
    let auth_router = SysAuthenticationRouter::init_authorization_router().await
        .layer(Extension(Arc::new(SysAuthService) as Arc<SysAuthService>))
        .layer(Extension(Arc::new(SysAuthorizationService) as Arc<SysAuthorizationService>));

    // 无服务注入
    let auth_router = apply_layers(
        auth_router,
        Services::None(std::marker::PhantomData::<()>),
        true,
        true,
        None,
        casbin.clone(),
        audience
    ).await;

    app = app.merge(auth_router);

    // 合并保护路由
    merge_router!(
        SysAuthenticationRouter::init_protected_router().await,
        SysAuthService,
        false,
        true,
        None
    );

    // 菜单路由
    merge_router!(SysMenuRouter::init_menu_router().await, SysMenuService, false, false, None);

    // 菜单保护路由
    merge_router!(
        SysMenuRouter::init_protected_menu_router().await,
        SysMenuService,
        true,
        true,
        None
    );

    // 用户路由
    merge_router!(SysUserRouter::init_user_router().await, SysUserService, true, true, None);
    // 领域路由
    merge_router!(SysDomainRouter::init_domain_router().await, SysDomainService, true, true, None);
    // 角色路由
    merge_router!(SysRoleRouter::init_role_router().await, SysRoleService, true, true, None);
    // 端点路由
    merge_router!(
        SysEndpointRouter::init_endpoint_router().await,
        SysEndpointService,
        true,
        true,
        None
    );
    // key路由
    merge_router!(
        SysAccessKeyRouter::init_access_key_router().await,
        SysAccessKeyService,
        true,
        true,
        None
    );
    // 登录日志路由
    merge_router!(
        SysLoginLogRouter::init_login_log_router().await,
        SysLoginLogService,
        true,
        true,
        None
    );
    // 操作日志路由
    merge_router!(
        SysOperationLogRouter::init_operation_log_router().await,
        SysOperationLogService,
        true,
        true,
        None
    );

    // 组织路由
    merge_router!(
        SysOrganizationRouter::init_organization_router().await,
        SysOrganizationService,
        false,
        false,
        None
    );

    // sandbox路由
    merge_router!(
        SysSandboxRouter::init_simple_sandbox_router().await,
        None,
        false,
        false,
        Some(simple_validation)
    );
    //sandbox路由
    merge_router!(
        SysSandboxRouter::init_complex_sandbox_router().await,
        None,
        false,
        false,
        Some(complex_validation)
    );

    // 后备路由
    app = app.fallback(handler_404);

    // 收集路由信息
    process_collected_routes().await;
    info!("Admin router initialization completed");
    app
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
// 收集路由信息
async fn process_collected_routes() {
    let routes = get_collected_routes().await;
    let endpoints: Vec<sys_endpoint::Model> = routes
        .into_iter()
        .map(|route| {
            let resource = route.path.split('/').nth(1).unwrap_or("").to_string();
            // 构建端点
            sys_endpoint::Model {
                id: generate_id(&route.path, &route.method.to_string()),
                path: route.path.clone(),
                method: route.method.to_string(),
                action: "rw".to_string(),
                resource,
                controller: route.service_name,
                summary: Some(route.summary),
                created_at: Local::now().naive_local(),
                updated_at: None,
            }
        })
        .collect();

    // 端点服务
    let endpoint_service = SysEndpointService;
    match endpoint_service.sync_endpoints(endpoints).await {
        Ok(_) => { info!("Endpoints synced successfully") }
        Err(e) => { error!("Failed to sync endpoints: {:?}", e) }
    }
}

fn generate_id(path: &str, method: &str) -> String {
    use std::{ collections::hash_map::DefaultHasher, hash::{ Hash, Hasher } };

    let mut hasher = DefaultHasher::new();
    format!("{}{}", path, method).hash(&mut hasher);
    format!("{:x}", hasher.finish())
}
