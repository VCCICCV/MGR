use std::{
    collections::HashMap,
    convert::Infallible,
    net::SocketAddr,
    task::{Context, Poll},
};

use axum::{
    body::{to_bytes, Body, Bytes},
    extract::{ConnectInfo, Request},
    response::Response,
    Extension,
};
use bytes::BytesMut;
use chrono::Local;
use futures::{future::BoxFuture, StreamExt};
use http::{Extensions, HeaderMap, Uri};
use serde_json::Value;
use server_constant::definition::consts::SystemEvent;
use server_global::global::{self, OperationLogContext};
use tower_layer::Layer;
use tower_service::Service;

use super::{auth::User, RequestId};

const USER_AGENT_HEADER: &str = "user-agent";
const UNKNOWN_REQUEST_ID: &str = "unknown";
const DEFAULT_BODY_CAPACITY: usize = 1024 * 16; // 16KB 默认缓冲区大小

#[derive(Clone)]
pub struct OperationLogLayer {
    pub enabled: bool,
}

impl OperationLogLayer {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl<S> Layer<S> for OperationLogLayer
where
    S: Service<Request<Body>, Response = Response<Body>, Error = Infallible>
        + Send
        + Clone
        + 'static,
{
    type Service = OperationLogMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        OperationLogMiddleware {
            inner: service,
            enabled: self.enabled,
        }
    }
}

#[derive(Clone)]
pub struct OperationLogMiddleware<S> {
    inner: S,
    enabled: bool,
}

impl<S> Service<Request<Body>> for OperationLogMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>, Error = Infallible>
        + Send
        + Clone
        + 'static,
    S::Future: Send + 'static,
{
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    type Response = Response<Body>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        if !self.enabled {
            let mut inner = self.inner.clone();
            return Box::pin(async move { inner.call(req).await });
        }

        let mut inner = self.inner.clone();
        Box::pin(async move {
            let start_time = Local::now().naive_local();
            let (parts, body) = req.into_parts();
            let headers = &parts.headers;
            let extensions = &parts.extensions;

            let (user_id, username, domain) = get_user_info(extensions);

            let request_id = extensions
                .get::<RequestId>()
                .map(ToString::to_string)
                .unwrap_or_else(|| UNKNOWN_REQUEST_ID.to_string());

            if let Ok(bytes) = buffer_body(body).await {
                let method = parts.method.to_string();
                let uri = parts.uri.to_string();
                let ip = get_client_ip(extensions, headers);
                let user_agent = get_user_agent(headers);
                let params = parse_query_params(&parts.uri);

                let req = Request::from_parts(parts, Body::from(bytes.clone()));
                let response = inner.call(req).await?;

                let (response_parts, response_body) = response.into_parts();
                let response_bytes = to_bytes(response_body, usize::MAX)
                    .await
                    .unwrap_or_default();

                let end_time = Local::now().naive_local();
                let duration = (end_time - start_time).num_milliseconds() as i32;

                let context = OperationLogContext {
                    user_id,
                    username,
                    domain,
                    module_name: "TODO".to_string(),
                    description: "TODO".to_string(),
                    request_id,
                    method,
                    url: uri,
                    ip,
                    user_agent,
                    params,
                    body: (!bytes.is_empty())
                        .then(|| serde_json::from_slice(&bytes).ok())
                        .flatten(),
                    response: serde_json::from_slice(&response_bytes).ok(),
                    start_time,
                    end_time,
                    duration,
                    created_at: start_time,
                };

                global::send_dyn_event(
                    SystemEvent::AuditOperationLoggedEvent.as_ref(),
                    Box::new(context),
                );

                Ok(Response::from_parts(
                    response_parts,
                    Body::from(response_bytes),
                ))
            } else {
                let mut inner = inner;
                inner.call(Request::from_parts(parts, Body::empty())).await
            }
        })
    }
}

/// 缓冲请求体，带容量限制
///
/// # 参数
/// * `body` - 请求体，类型为 axum 的 Body
///
/// # 返回值
/// * `Result<Bytes, Box<dyn std::error::Error + Send + Sync>>` -
///   成功返回缓冲的字节数据，失败返回错误
///
/// # 错误
/// * 当请求体大小超过 MAX_SIZE (32KB) 时返回错误
/// * 当读取请求体流失败时返回错误
#[inline]
async fn buffer_body(body: Body) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
    const MAX_SIZE: usize = DEFAULT_BODY_CAPACITY * 2;
    let mut bytes = BytesMut::with_capacity(DEFAULT_BODY_CAPACITY);

    let mut stream = body.into_data_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        if bytes.len() + chunk.len() > MAX_SIZE {
            return Err("Request body too large".into());
        }
        bytes.extend_from_slice(&chunk);
    }

    Ok(bytes.freeze())
}

/// 从请求头获取用户代理
///
/// # 参数
/// * `headers` - HTTP 请求头映射
///
/// # 返回值
/// * `Option<String>` - 成功返回用户代理字符串，未找到或解析失败返回 None
#[inline(always)]
fn get_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get(USER_AGENT_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned)
}

/// 获取客户端 IP，优先从扩展中获取
///
/// # 参数
/// * `extensions` - 请求扩展
/// * `headers` - HTTP 请求头映射
///
/// # 返回值
/// * `String` - 客户端 IP 地址字符串
#[inline(always)]
fn get_client_ip(extensions: &Extensions, headers: &HeaderMap) -> String {
    use std::net::IpAddr;

    if let Some(Extension(ConnectInfo(addr))) =
        extensions.get::<Extension<ConnectInfo<SocketAddr>>>()
    {
        return match addr.ip() {
            IpAddr::V4(ip) => ip.to_string(),
            IpAddr::V6(ip) => ip.to_string(),
        };
    }

    super::util::ClientIp::get_real_ip(headers)
}

/// 从扩展中获取用户信息元组
///
/// # 参数
/// * `extensions` - 请求扩展
///
/// # 返回值
/// * `(Option<String>, Option<String>, Option<String>)` - (用户ID, 用户名,
///   域名) 的元组
#[inline(always)]
fn get_user_info(extensions: &Extensions) -> (Option<String>, Option<String>, Option<String>) {
    let Some(user) = extensions.get::<User>() else {
        return Default::default();
    };

    (
        Some(user.user_id()),
        Some(user.username()),
        Some(user.domain()),
    )
}

/// 解析 URI 查询参数为 JSON 值
///
/// # 参数
/// * `uri` - HTTP 请求 URI
///
/// # 返回值
/// * `Option<Value>` - 成功返回解析后的 JSON 值，失败返回 None
#[inline(always)]
fn parse_query_params(uri: &Uri) -> Option<Value> {
    let query = uri.query()?;

    if query.is_empty() {
        return Some(Value::Object(Default::default()));
    }

    let capacity = query.matches('&').count() + 1;
    let mut params = HashMap::with_capacity(capacity);

    form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .for_each(|(k, v)| {
            params.insert(k, v);
        });

    serde_json::to_value(params).ok()
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use axum::{
        body::{Body, HttpBody},
        http::{Method, Request, StatusCode},
    };
    use serde_json::json;

    use super::*;
    use crate::web::auth::{Claims, User};

    /// 创建测试用户
    fn create_test_user() -> User {
        let claims = Claims::new(
            "test-user".to_string(),
            "test-aud".to_string(),
            "test".to_string(),
            vec!["admin".to_string()],
            "test-domain".to_string(),
            None,
        );
        User::from(claims)
    }

    /// 创建测试请求
    fn create_request(method: Method, uri: &str, body: Option<Value>) -> Request<Body> {
        let mut builder = Request::builder()
            .method(method)
            .uri(uri)
            .header("user-agent", "test-agent")
            .header("X-Real-IP", "192.168.1.1")
            .extension(create_test_user());

        if body.is_some() {
            builder = builder.header("content-type", "application/json");
        }

        builder
            .body(match body {
                Some(json) => Body::from(serde_json::to_vec(&json).unwrap()),
                None => Body::empty(),
            })
            .unwrap()
    }

    /// 验证上下文
    async fn assert_context(method: &str, uri: &str, params: Option<Value>, body: Option<Value>) {
        let ctx = OperationLogContext::get()
            .await
            .expect("Context should exist");
        println!("验证上下文: {} {}", method, uri);
        println!("参数: {:?}", params);
        println!("请求体: {:?}", body);

        assert_eq!(ctx.method, method);
        assert_eq!(ctx.url, uri);
        assert_eq!(ctx.params, params);
        assert_eq!(ctx.body, body);
        assert_eq!(ctx.user_agent, Some("test-agent".to_string()));
        assert_eq!(ctx.ip, "192.168.1.1");
        assert_eq!(ctx.user_id, Some("test-user".to_string()));
    }

    #[tokio::test]
    async fn test_operation_log_completeness() {
        let test_cases = vec![
            // 基础场景
            (Method::GET, "/test", None, None),
            (
                Method::GET,
                "/test?key=value",
                Some(json!({"key": "value"})),
                None,
            ),
            (Method::POST, "/test", None, Some(json!({"data": "test"}))),
            // 边界场景
            (Method::GET, "/test?", Some(json!({})), None), // 空查询参数
            (Method::POST, "/test", None, Some(json!({}))), // 空请求体
            // 复杂场景
            (
                Method::POST,
                "/test?a=1&b=2",
                Some(json!({"a": "1", "b": "2"})),
                Some(json!({"nested": {"data": "test"}})),
            ),
            // 特殊字符场景
            (
                Method::GET,
                "/test?key=hello%20world",
                Some(json!({"key": "hello world"})),
                None,
            ),
            // 其他 HTTP 方法
            (
                Method::PUT,
                "/test?type=update",
                Some(json!({"type": "update"})),
                Some(json!({"status": "done"})),
            ),
            (Method::DELETE, "/test/123", None, None),
            (Method::PATCH, "/test", None, Some(json!({"op": "replace"}))),
            // 大小写混合场景
            (
                Method::GET,
                "/TEST?Key=Value",
                Some(json!({"Key": "Value"})),
                None,
            ),
        ];

        let service = tower::service_fn(|_req: Request<Body>| async move {
            Ok::<_, Infallible>(Response::new(Body::from("ok")))
        });

        for (method, uri, params, body) in test_cases {
            OperationLogContext::clear().await;
            println!("\n▶ 测试场景: {} {}", method, uri);
            if let Some(p) = &params {
                println!("  查询参数: {}", p);
            }
            if let Some(b) = &body {
                println!("  请求体: {}", b);
            }

            let mut middleware = OperationLogMiddleware {
                inner: service.clone(),
                enabled: true,
            };

            let request = create_request(method.clone(), uri, body.clone());
            let _ = middleware.call(request).await.unwrap();

            assert_context(&method.to_string(), uri, params, body).await;
        }
    }

    #[tokio::test]
    async fn test_operation_log_error_cases() {
        println!("\n=== Testing Error Cases ===");

        let service = tower::service_fn(|req: Request<Body>| async move {
            // 检查请求体大小
            let content_length = req
                .headers()
                .get("content-length")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(0);

            if content_length > DEFAULT_BODY_CAPACITY * 2 {
                Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Request body too large"))
                    .unwrap())
            } else {
                Ok(Response::new(Body::from("ok")))
            }
        });

        let test_cases = vec![(
            Method::POST,
            "/test",
            Some(json!({
                "large": "x".repeat(DEFAULT_BODY_CAPACITY * 3)
            })),
            StatusCode::BAD_REQUEST,
        )];

        for (method, uri, body, expected_status) in test_cases {
            println!("\n▶ 测试错误场景: {:?}", expected_status);

            let mut middleware = OperationLogMiddleware {
                inner: service.clone(),
                enabled: true,
            };

            let mut request = create_request(method, uri, body);

            // 添加 content-length 头
            if let Some(body) = request.body().size_hint().upper() {
                request
                    .headers_mut()
                    .insert("content-length", body.to_string().parse().unwrap());
            }

            let response = middleware.call(request).await.unwrap();
            assert_eq!(response.status(), expected_status);
        }
    }

    #[tokio::test]
    async fn test_disabled_middleware() {
        println!("\n=== Testing Disabled Middleware ===");
        OperationLogContext::clear().await;

        let service = tower::service_fn(|_req: Request<Body>| async move {
            Ok::<_, Infallible>(Response::new(Body::from("ok")))
        });

        let mut middleware = OperationLogMiddleware {
            inner: service,
            enabled: false,
        };

        let request = create_request(Method::POST, "/test", Some(json!({"test": true})));
        let response = middleware.call(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert!(OperationLogContext::get().await.is_none());
    }
}
