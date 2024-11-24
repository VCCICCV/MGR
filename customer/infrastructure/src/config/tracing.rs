use domain::model::reponse::error::AppResult;
use tracing::{ subscriber, Subscriber };
use tracing_appender::{ non_blocking::WorkerGuard, rolling::{ RollingFileAppender, Rotation } };
use tracing_log::LogTracer;
use tracing_subscriber::{ fmt::{ self, MakeWriter }, layer::SubscriberExt, EnvFilter, Registry };
use tracing_bunyan_formatter::{ BunyanFormattingLayer, JsonStorageLayer };
// 创建订阅者
fn create_subscriber<W>(
    name: &str,
    env_filter: EnvFilter,
    writer: W
) -> impl Subscriber + Sync + Send
    where W: for<'a> MakeWriter<'a> + Send + Sync + 'static
{
    // 创建格式化层
    let fmt_layer = fmt::Layer
        ::default()
        // 显示日志来源的目标信息
        .with_target(true)
        // 显示线程 ID
        .with_thread_ids(true)
        // 线程名称信息
        .with_thread_names(true)
        // 启用 ANSI 转义码来支持彩色输出
        .with_ansi(true)
        .compact();
    // 注册订阅者
    Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        // 以 JSON 格式进行处理
        .with(JsonStorageLayer)
        // 以文本格式进行输出到控制台
        .with(BunyanFormattingLayer::new(name.into(), std::io::stdout))
        // 以文本格式进行输出到文件
        .with(BunyanFormattingLayer::new(name.into(), writer))
}
// 初始化订阅者
pub fn init_subscriber<S>(subscriber: S) -> anyhow::Result<()>
    where S: Subscriber + Send + Sync + 'static
{
    LogTracer::init()?;
    subscriber::set_global_default(subscriber)?;
    Ok(())
}
// 初始化并返回文件句柄
pub fn init() -> AppResult<WorkerGuard> {
    // 构建每日日志，前缀为app.log
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
    // 构建非阻塞日志
    let (file_appender, file_appender_guard) = tracing_appender::non_blocking(file_appender);
    // 初始化订阅者，从环境变量设置日志级别
    init_subscriber(create_subscriber("app", EnvFilter::from_default_env(), file_appender))?;
    Ok(file_appender_guard)
}

// use serde::Deserialize;
// #[derive(Debug, Deserialize, Clone)]
// pub struct TracingConfig {
//     log_level: String,
// }
// impl TracingConfig {
//     // 获取地址
//     pub fn get_log_level(&self) -> String {
//         format!("{}", self.log_level)
//     }
// }
// #[cfg(test)]
// pub mod tests {

//     use super::*;

//     #[test]
//     pub fn app_config_http_addr_test() {
//         let config = TracingConfig {
//             log_level: "debug".to_string(),
//         };
//         assert_eq!(config.get_log_level(), "debug");
//     }
// }
