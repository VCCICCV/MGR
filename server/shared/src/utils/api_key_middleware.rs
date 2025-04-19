use axum::{
    body::Body,
    extract::Request,
    http::{ HeaderMap, Uri },
    middleware::Next,
    response::IntoResponse,
};
use once_cell::sync::Lazy;
use std::{ collections::HashSet, sync::RwLock };

use crate::{ constant::SystemEvent, global, res::Res };

use super::{ ApiKeyEvent, ComplexApiKeyValidator, SimpleApiKeyValidator };

/// 保护路由
///
/// 需要apikey 验证
static PROTECTED_PATHS: Lazy<RwLock<HashSet<String>>> = Lazy::new(|| RwLock::new(HashSet::new()));

/// APi KEY的位置
#[derive(Clone, Copy, PartialEq)]
pub enum ApiKeySource {
    /// From request header.
    Header,
    /// From query parameter.
    Query,
}

///用于简单API密钥验证的配置。
///此结构保存简单API密钥验证的配置。
#[derive(Clone)]
pub struct SimpleApiKeyConfig {
    /// Source location of API key.
    pub source: ApiKeySource,
    /// Name of API key parameter.
    pub key_name: String,
}

impl Default for SimpleApiKeyConfig {
    fn default() -> Self {
        Self {
            source: ApiKeySource::Header,
            key_name: "x-api-key".to_string(),
        }
    }
}

///配置具有签名的复杂API密钥验证。
///此结构保存具有签名的复杂API密钥验证的配置。
#[derive(Clone)]
pub struct ComplexApiKeyConfig {
    /// Access key ID parameter name.
    pub key_name: String,
    /// Timestamp parameter name.
    pub timestamp_name: String,
    /// Nonce parameter name.
    pub nonce_name: String,
    /// Signature parameter name.
    pub signature_name: String,
}

impl Default for ComplexApiKeyConfig {
    fn default() -> Self {
        Self {
            key_name: "AccessKeyId".to_string(),
            timestamp_name: "timestamp".to_string(),
            nonce_name: "nonce".to_string(),
            signature_name: "signature".to_string(),
        }
    }
}

/// API密钥验证策略。
/// 此列举定义了可能的API密钥验证策略。
#[derive(Clone)]
pub enum ApiKeyValidation {
    /// Simple API key validation.
    Simple(SimpleApiKeyValidator, SimpleApiKeyConfig),
    /// Complex API key validation with signature.
    Complex(ComplexApiKeyValidator, ComplexApiKeyConfig),
}

///添加需要API密钥验证的受保护路由的路径。
///该函数向受保护的路径集添加路径。
#[allow(dead_code)]
pub fn protect_route(path: &str) {
    if let Ok(mut paths) = PROTECTED_PATHS.write() {
        paths.insert(path.to_string());
    }
}

///检查URI路径是否需要API密钥验证。
///此函数检查给定的URI路径是否在受保护的路径集中
#[inline]
fn is_protected_path(uri: &Uri) -> bool {
    if let Ok(paths) = PROTECTED_PATHS.read() {
        let path = uri.path();
        paths.contains(path.strip_suffix('/').unwrap_or(path))
    } else {
        false
    }
}

/// API密钥验证中间件
///此中间件检查API密钥对于给定请求是否有效
#[inline]
pub async fn api_key_middleware(
    validator: ApiKeyValidation,
    req: Request<Body>,
    next: Next
) -> impl IntoResponse {
    if !is_protected_path(req.uri()) {
        return next.run(req).await.into_response();
    }

    match validate_request(&validator, &req) {
        Ok(true) => next.run(req).await.into_response(),
        Ok(false) => Res::<()>::with_err("Invalid API key or signature").into_response(),
        Err(e) => Res::<()>::with_err(e).into_response(),
    }
}

///从请求头获取值。
///该函数从请求头中检索头的值。
#[inline]
fn get_header_value<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name).and_then(|v| v.to_str().ok())
}
///从查询参数获取值。
///该函数从请求查询中检索查询参数的值。
#[inline]
fn get_query_value<'a>(params: &'a [(String, String)], name: &str) -> Option<&'a str> {
    params
        .iter()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.as_str())
}

///在请求中删除API密钥。
///此函数验证给定请求中的API密钥。
#[inline]
fn validate_request(
    validator: &ApiKeyValidation,
    req: &Request<Body>
) -> Result<bool, &'static str> {
    let headers = req.headers();
    let query = req.uri().query().unwrap_or("");
    let params = if !query.is_empty() { parse_query(query) } else { Vec::new() };

    match validator {
        ApiKeyValidation::Simple(validator, config) => {
            let api_key = (
                match config.source {
                    ApiKeySource::Header => get_header_value(headers, &config.key_name),
                    ApiKeySource::Query => get_query_value(&params, &config.key_name),
                }
            ).ok_or("Missing API key")?;

            global::send_dyn_event(
                SystemEvent::AuthApiKeyValidatedEvent.as_ref(),
                Box::new(ApiKeyEvent {
                    api_key: api_key.to_owned(),
                })
            );
            Ok(validator.validate_key(api_key))
        }
        ApiKeyValidation::Complex(validator, config) => {
            let api_key = get_query_value(&params, &config.key_name).ok_or("Missing AccessKeyId")?;

            let timestamp = get_query_value(&params, &config.timestamp_name)
                .ok_or("Missing timestamp")?
                .parse::<i64>()
                .map_err(|_| "Invalid timestamp")?;

            let nonce = get_query_value(&params, &config.nonce_name).ok_or("Missing nonce")?;

            let signature = get_query_value(&params, &config.signature_name).ok_or(
                "Missing signature"
            )?;

            let params_for_signing: Vec<(String, String)> = params
                .iter()
                .filter(|(k, _)| k != &config.signature_name)
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();

            global::send_dyn_event(
                SystemEvent::AuthApiKeyValidatedEvent.as_ref(),
                Box::new(ApiKeyEvent {
                    api_key: api_key.to_owned(),
                })
            );
            Ok(
                validator.validate_signature(
                    api_key,
                    &params_for_signing,
                    signature,
                    timestamp,
                    nonce
                )
            )
        }
    }
}

///将查询字符串解析为关键字-值对。
///该函数将查询字符串解析为琴键-值对的载体。
#[inline]
fn parse_query(query: &str) -> Vec<(String, String)> {
    let capacity = query.matches('&').count() + 1;
    let mut params = Vec::with_capacity(capacity);

    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            if !k.is_empty() && !v.is_empty() {
                params.push((k.to_string(), v.to_string()));
            }
        }
    }

    params
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{ SystemTime, UNIX_EPOCH };

    #[test]
    fn test_api_key_sign() {
        let validator = ComplexApiKeyValidator::new(None);
        validator.add_key_secret("test-access-key".to_string(), "test-secret-key".to_string());

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        let nonce = format!("nonce_{}", timestamp);

        let mut params = vec![
            ("AccessKeyId".to_string(), "test-access-key".to_string()),
            ("param1".to_string(), "value1".to_string()),
            ("param2".to_string(), "value2".to_string()),
            ("t".to_string(), timestamp.to_string()),
            ("n".to_string(), nonce.clone())
        ];

        params.sort_by(|a, b| a.0.cmp(&b.0));

        let signing_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        let signature = validator.calculate_signature(&signing_string, "test-secret-key");

        println!("URL with signature: /api/url?{}&sign={}", signing_string, signature);
    }
}
