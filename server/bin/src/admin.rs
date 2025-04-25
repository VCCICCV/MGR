use std::net::SocketAddr;
use tokio::net::TcpListener;
use initialize;
#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    // 加载.env 环境配置文件，成功返回包含的值，失败返回None
    dotenvy::dotenv().ok();
    println!("=== 当前环境变量 ===");
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
    let config_path = if cfg!(debug_assertions) {
        // 开发模式使用测试配置
        "../settings/development.toml"
    } else {
        // 发布模式使用生产配置
        "../settings/production.toml"
    };
    // 初始化tracing
    initialize::initialize_log_tracing().await;
    // 初始化配置
    initialize::initialize_config(config_path).await;
    // 初始化ip
    let _ = initialize::init_xdb().await;
    // 初始化数据库
    initialize::init_primary_connection().await;
    initialize::init_db_pools().await;
    // 初始化验证器
    initialize::initialize_keys_and_validation().await;
    // 初始化事件通道
    initialize::initialize_event_channel().await;

    // 初始化redis
    initialize::init_primary_redis().await;
    initialize::init_redis_pools().await;

    // 初始化路由
    let app = initialize::initialize_admin_router().await;

    //需要初始化验证器init_validators之后才能初始化访问密钥
    initialize::initialize_access_key().await;

    let addr = match initialize::get_server_address().await {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("Failed to get server address: {}", e);
            return;
        }
    };

    // run it
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
