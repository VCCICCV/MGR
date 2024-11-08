use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    username: String,
    password: String,
    port: u16,
    host: String,
    database_name: String,
}

impl DatabaseConfig {
    pub fn get_url(&self) -> String {
        Self::create_url(&self.username, &self.password, &self.host, self.port, &self.database_name)
    }
    // 创建数据库连接字符串
    pub fn create_url(
        username: &str,
        password: &str,
        host: &str,
        port: u16,
        database_name: &str
    ) -> String {
        format!("postgres://{username}:{password}@{host}:{port}/{database_name}")
    }
}
