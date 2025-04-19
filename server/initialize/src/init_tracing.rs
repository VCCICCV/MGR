use opentelemetry_otlp::WithExportConfig;
use shared::error::AppError;
use tracing::{ subscriber, Subscriber };
use tracing_appender::{ non_blocking::WorkerGuard, rolling::{ RollingFileAppender, Rotation } };
use tracing_log::LogTracer;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{ fmt::{ self, MakeWriter }, layer::SubscriberExt, EnvFilter, Registry };
use tracing_bunyan_formatter::{ BunyanFormattingLayer, JsonStorageLayer };
use opentelemetry::trace::TracerProvider;
use std::time::Duration;


// 创建订阅者
fn create_subscriber<W>(
    name: &str,
    env_filter: EnvFilter,
    writer: W
) -> impl Subscriber + Sync + Send
    where W: for<'a> MakeWriter<'a> + Send + Sync + 'static
{
    // OTLP 导出器配置
    let exporter = opentelemetry_otlp::SpanExporter
        ::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317")
        .with_timeout(Duration::from_secs(3))
        .build()
        .expect("OTLP exporter failed");
    // 追踪导出器
    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider
        ::builder()
        .with_batch_exporter(exporter)
        .build();
    // 创建追踪器对象
    let tracer = tracer_provider.tracer(name.to_string());
    // 创建控制台格式化层
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
    // 文件输出层
    let file_layer = BunyanFormattingLayer::new(name.into(), writer);
    // 注册订阅者
    Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .with(OpenTelemetryLayer::new(tracer))
        // 以 JSON 格式进行处理
        .with(JsonStorageLayer)
        // 以文本格式进行输出到控制台
        .with(BunyanFormattingLayer::new(name.into(), std::io::stdout))
        // 以文本格式进行输出到文件
        .with(file_layer)
}
// 初始化订阅者
pub fn init_subscriber<S>(subscriber: S) -> Result<(),AppError>
    where S: Subscriber + Send + Sync + 'static
{
    LogTracer::init().expect("Failed to set logger");
    subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    Ok(())
}
// 初始化并返回文件句柄
pub fn init() ->Result<WorkerGuard,Box<dyn std::error::Error>> {
    // 构建每日日志，前缀为app.log
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "../logs", "app.log");
    // 构建非阻塞日志
    let (file_appender, file_appender_guard) = tracing_appender::non_blocking(file_appender);
    // 初始化订阅者，从环境变量设置日志级别
    init_subscriber(create_subscriber("app", EnvFilter::from_default_env(), file_appender))?;
    Ok(file_appender_guard)
}
