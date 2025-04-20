use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabasesInstancesConfig {
    pub name: String,
    pub database: DatabaseConfig,
}
