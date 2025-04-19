use axum::http::HeaderMap;

/// 客户端 IP 地址处理工具
///
/// 用于从 HTTP 请求头中获取真实的客户端 IP 地址。
/// 支持多种代理服务器和 CDN 的请求头格式。
pub struct ClientIp;

impl ClientIp {
    /// 从请求头中获取真实的客户端 IP 地址
    ///
    /// # 参数
    /// * `headers` - HTTP 请求头
    ///
    /// # 返回值
    /// 返回客户端 IP 地址字符串，如果无法获取则返回 "unknown"
    ///
    /// # 示例
    /// ```
    /// use axum::http::HeaderMap;
    /// use shared::utils::ip::ClientIp;
    ///
    /// let mut headers = HeaderMap::new();
    /// headers.insert("X-Real-IP", "192.168.1.1".parse().unwrap());
    ///
    /// let ip = ClientIp::get_real_ip(&headers);
    /// assert_ne!(ip, "unknown");
    /// ```
    pub fn get_real_ip(headers: &HeaderMap) -> String {
        // 按优先级检查请求头
        // 不同的代理服务器和 CDN 可能使用不同的请求头来传递客户端 IP
        let ip_headers = [
            "X-Real-IP", // Nginx 代理
            "X-Forwarded-For", // 标准代理头
            "CF-Connecting-IP", // Cloudflare
            "True-Client-IP", // Akamai 和 Cloudflare
            "X-Client-IP", // 常用代理头
            "Fastly-Client-IP", // Fastly CDN
            "X-Cluster-Client-IP", // GCP/AWS Load Balancer
            "X-Original-Forwarded-For", // AWS ALB/ELB
        ];

        // 遍历所有可能的请求头
        for header_name in ip_headers {
            if let Some(ip_header) = headers.get(header_name) {
                if let Ok(ip_str) = ip_header.to_str() {
                    // X-Forwarded-For 等头可能包含多个 IP，格式如：client, proxy1, proxy2
                    // 我们取第一个，因为它通常是最原始的客户端 IP
                    let real_ip = ip_str.split(',').next().unwrap_or("").trim();
                    if !real_ip.is_empty() {
                        return real_ip.to_string();
                    }
                }
            }
        }

        // 如果没有找到任何有效的 IP，返回 unknown
        "unknown".to_string()
    }

    /// 检查 IP 地址是否有效
    ///
    /// # 参数
    /// * `ip` - IP 地址字符串
    ///
    /// # 返回值
    /// 如果 IP 地址有效则返回 true，否则返回 false
    pub fn is_valid_ip(ip: &str) -> bool {
        if ip == "unknown" {
            return false;
        }

        // 简单的 IPv4 验证
        ip
            .split('.')
            .filter_map(|octet| octet.parse::<u8>().ok())
            .count() == 4
    }

    /// 获取请求头中的所有 IP 相关信息
    ///
    /// # 参数
    /// * `headers` - HTTP 请求头
    ///
    /// # 返回值
    /// 返回一个包含所有 IP 相关请求头信息的 Vec
    pub fn get_all_ip_headers(headers: &HeaderMap) -> Vec<(String, String)> {
        let ip_headers = [
            "X-Real-IP",
            "X-Forwarded-For",
            "CF-Connecting-IP",
            "True-Client-IP",
            "X-Client-IP",
            "Fastly-Client-IP",
            "X-Cluster-Client-IP",
            "X-Original-Forwarded-For",
        ];

        ip_headers
            .iter()
            .filter_map(|&header_name| {
                headers.get(header_name).and_then(|value| {
                    value
                        .to_str()
                        .ok()
                        .map(|v| (header_name.to_string(), v.to_string()))
                })
            })
            .collect()
    }

    /// 从 X-Forwarded-For 头中获取代理链路
    ///
    /// # 参数
    /// * `headers` - HTTP 请求头
    ///
    /// # 返回值
    /// 返回代理链路中的所有 IP 地址，如果没有则返回空 Vec
    pub fn get_proxy_chain(headers: &HeaderMap) -> Vec<String> {
        headers
            .get("X-Forwarded-For")
            .and_then(|h| h.to_str().ok())
            .map(|ip_str|
                ip_str
                    .split(',')
                    .map(|ip| ip.trim().to_string())
                    .collect()
            )
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_real_ip() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Real-IP", "192.168.1.1".parse().unwrap());
        assert_eq!(ClientIp::get_real_ip(&headers), "192.168.1.1");
    }

    #[test]
    fn test_is_valid_ip() {
        assert!(ClientIp::is_valid_ip("192.168.1.1"));
        assert!(!ClientIp::is_valid_ip("unknown"));
        assert!(!ClientIp::is_valid_ip("256.256.256.256"));
    }

    #[test]
    fn test_get_proxy_chain() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Forwarded-For", "192.168.1.1, 10.0.0.1, 172.16.0.1".parse().unwrap());
        let chain = ClientIp::get_proxy_chain(&headers);
        assert_eq!(chain.len(), 3);
        assert_eq!(chain[0], "192.168.1.1");
    }
}
