use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct EsConfig {
    host: String,
    port: u16,
}
impl EsConfig {
    // 获取redis连接地址
    pub fn get_url(&self) -> String {
        format!("http://{host}:{port}", host = self.host, port = self.port)
    }
}
