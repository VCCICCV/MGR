use std::sync::Arc;

use axum::{ body::Body, response::IntoResponse, Extension, Router };
use axum_casbin::CasbinAxumLayer;
use config::AppConfig;
use http::{ Request, StatusCode };
use middleware::jwt::jwt_auth_middleware;
use shared::{
    constant::Audience,
    global::{ clear_routes, get_collected_routes, get_config },
    request_id::{ RequestId, RequestIdLayer },
    utils::{
        self,
        add_key,
        api_key_middleware,
        protect_route,
        ApiKeySource,
        ApiKeyValidation,
        ComplexApiKeyConfig,
        SimpleApiKeyConfig,
        ValidatorType,
    },
};
use tower_http::trace::TraceLayer;
use tracing::{ error, info, info_span };
use chrono::Local;
use crate::init_casbin::initialize_casbin;

// 服务实例
#[derive(Clone)]
pub enum Services<T: Send + Sync + 'static> {
    None(std::marker::PhantomData<T>),
    Single(Arc<T>),
}
// 动态添加中间件
async fn apply_layers<T: Send + Sync + 'static>(
    router: Router,
    services: Services<T>,
    need_casbin: bool,
    need_auth: bool,
    api_validation: Option<ApiKeyValidation>,
    casbin: Option<CasbinAxumLayer>,
    audience: Audience
) -> Router {
    let mut router = match services {
        Services::None(_) => router,
        Services::Single(service) => router.layer(Extension(service)),
    };

    // 添加追中间件
    router = router
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".into());
                info_span!(
                    "[mgr] >>>>>> request",
                    id = %request_id,
                    method = %request.method(),
                    uri = %request.uri(),
                )
            })
        )
        .layer(RequestIdLayer);

    // 需要验证则添加casbin
    if need_casbin {
        if let Some(casbin) = casbin {
            router = router.layer(Extension(casbin.clone())).layer(casbin);
        }
    }

    // 验证请求
    if let Some(validation) = api_validation {
        router = router.layer(
            axum::middleware::from_fn(move |req, next| {
                api_key_middleware(validation.clone(), req, next)
            })
        );
    }

    // 需要鉴权添加jwt中间件
    if need_auth {
        router = router.layer(
            axum::middleware::from_fn(move |req, next| {
                jwt_auth_middleware(req, next, audience.as_str())
            })
        );
    }

    router
}
// 初始化adming router
pub async fn initialize_admin_router() -> Router {
    // 清除路由
    clear_routes().await;
    info!("Initializing admin router");

    // 加载配置
    let app_config = get_config::<AppConfig>().await.unwrap();
    // 初始化casbin
    let casbin_layer = initialize_casbin(
        "../../static/casbin/rbac_with_domains_model.conf",
        app_config.database.url.as_str()
    ).await.expect("Initializing casbin error");

    // 初始化验证器
    utils::init_validators(None).await;

    // 简单验证
    let simple_validation = {
        let validator = utils::get_simple_validator().await;
        utils::add_key(ValidatorType::Simple, "test-api-key", None).await;
        ApiKeyValidation::Simple(validator, SimpleApiKeyConfig {
            source: ApiKeySource::Header,
            key_name: "x-api-key".to_string(),
        })
    };

    // 复杂验证
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
    // 路由
    let mut app = Router::new();

    // 合并路由声明宏
    macro_rules! merge_router {
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

    // 鉴权服务
    merge_router!(
        SysAuthenticationRouter::init_authentication_router().await,
        SysAuthService,
        false,
        false,
        None
    );

    // 鉴权路由
    let auth_router = SysAuthenticationRouter::init_authorization_router().await
        .layer(Extension(Arc::new(SysAuthService) as Arc<SysAuthService>))
        .layer(Extension(Arc::new(SysAuthorizationService) as Arc<SysAuthorizationService>));

    // 路由配置
    let auth_router = apply_layers(
        auth_router,
        Services::None(std::marker::PhantomData::<()>),
        true,
        true,
        None,
        casbin.clone(),
        audience
    ).await;

    // 合并鉴权
    app = app.merge(auth_router);

    merge_router!(
        SysAuthenticationRouter::init_protected_router().await,
        SysAuthService,
        false,
        true,
        None
    );

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
    // Accesskey路由
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

    // sandbox
    merge_router!(
        SysSandboxRouter::init_simple_sandbox_router().await,
        None,
        false,
        false,
        Some(simple_validation)
    );
    merge_router!(
        SysSandboxRouter::init_complex_sandbox_router().await,
        None,
        false,
        false,
        Some(complex_validation)
    );

    // 404处理
    app = app.fallback(handler_404);

    // 收集路由信息
    process_collected_routes().await;
    info!("Admin router initialization completed");

    app
}
// 404处理
async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
// 收集路由信息
async fn process_collected_routes() {
    let routes = get_collected_routes().await;
    let endpoints: Vec<SysEndpoint> = routes
        .into_iter()
        .map(|route| {
            let resource = route.path.split('/').nth(1).unwrap_or("").to_string();
            SysEndpoint {
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

    let endpoint_service = SysEndpointService;
    match endpoint_service.sync_endpoints(endpoints).await {
        Ok(_) => { info!("Endpoints synced successfully") }
        Err(e) => { error!("Failed to sync endpoints: {:?}", e) }
    }
}

// 生成id
fn generate_id(path: &str, method: &str) -> String {
    use std::{ collections::hash_map::DefaultHasher, hash::{ Hash, Hasher } };

    let mut hasher = DefaultHasher::new();
    format!("{}{}", path, method).hash(&mut hasher);
    format!("{:x}", hasher.finish())
}
