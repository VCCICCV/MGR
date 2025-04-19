
pub mod init_database;
pub mod init_redis;
pub mod init_kafka;
pub mod init_tracing;
pub mod init_router;
pub mod init_config;
pub mod init_casbin;
pub mod init_event_channel;
pub mod init_jwt;
// // 传输配置文件到客户端
// pub trait ClientBuilder: Sized {
//     fn build_from_config(config: &AppConfig) -> Result<Self,AppError>;
// }
