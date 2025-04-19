use std::{ any::{ Any, TypeId }, collections::HashMap, pin::Pin, sync::Arc };

use chrono::NaiveDateTime;
use jsonwebtoken::{ DecodingKey, EncodingKey, Validation };
use once_cell::sync::Lazy;
use redis::Client;
use redis::cluster::ClusterClient;
use reqwest::Method;
use sea_orm::DatabaseConnection;
use serde_json::Value;
use tokio::sync::{ mpsc, Mutex, OnceCell, RwLock };
use tracing::info;
//*****************************************************************************
// 全局配置
//*****************************************************************************

pub static GLOBAL_CONFIG: Lazy<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>> = Lazy::new(||
    RwLock::new(HashMap::new())
);
// 初始化全局配置
pub async fn init_config<T: 'static + Any + Send + Sync>(config: T) {
    let mut context = GLOBAL_CONFIG.write().await;
    context.insert(TypeId::of::<T>(), Arc::new(config));
}
// 获取全局配置
pub async fn get_config<T: 'static + Any + Send + Sync>() -> Option<Arc<T>> {
    let context = GLOBAL_CONFIG.read().await;
    context.get(&TypeId::of::<T>()).and_then(|config| config.clone().downcast::<T>().ok())
}
//*****************************************************************************
// 数据库连接
//*****************************************************************************

pub static GLOBAL_PRIMARY_DB: Lazy<RwLock<Option<Arc<DatabaseConnection>>>> = Lazy::new(||
    RwLock::new(None)
);
pub static GLOBAL_DB_POOL: Lazy<RwLock<HashMap<String, Arc<DatabaseConnection>>>> = Lazy::new(||
    RwLock::new(HashMap::new())
);
// Redis连接
#[derive(Clone)]
pub enum RedisConnection {
    Single(Arc<Client>),
    Cluster(Arc<ClusterClient>),
}
pub static GLOBAL_PRIMARY_REDIS: Lazy<RwLock<Option<RedisConnection>>> = Lazy::new(||
    RwLock::new(None)
);

pub static GLOBAL_REDIS_POOL: Lazy<RwLock<HashMap<String, RedisConnection>>> = Lazy::new(||
    RwLock::new(HashMap::new())
);
//*****************************************************************************
// JWT 密钥和验证
//*****************************************************************************
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static KEYS: OnceCell<Arc<Mutex<Keys>>> = OnceCell::const_new();
pub static VALIDATION: OnceCell<Arc<Mutex<Validation>>> = OnceCell::const_new();
//*****************************************************************************
// 事件通道
//*****************************************************************************

/// 事件通道条目
struct DynChannelEntry {
    name: String,
    tx: mpsc::UnboundedSender<Box<dyn Any + Send>>,
}

/// 事件通道管理器
struct EventChannels {
    string_tx: mpsc::UnboundedSender<String>,
    dyn_channels: Vec<DynChannelEntry>,
}

static EVENT_CHANNELS: Lazy<Arc<Mutex<EventChannels>>> = Lazy::new(|| {
    let (string_tx, _) = mpsc::unbounded_channel();
    Arc::new(
        Mutex::new(EventChannels {
            string_tx,
            dyn_channels: Vec::new(),
        })
    )
});

type DynFuture = dyn Future<Output = ()> + Send + 'static;
type StringListener = Box<dyn FnOnce(mpsc::UnboundedReceiver<String>) -> Pin<Box<DynFuture>>>;
type DynListener = (
    String,
    Box<dyn Fn(mpsc::UnboundedReceiver<Box<dyn Any + Send>>) -> Pin<Box<DynFuture>>>,
);

/// 获取字符串事件发送器
#[inline]
pub async fn get_string_sender() -> mpsc::UnboundedSender<String> {
    EVENT_CHANNELS.lock().await.string_tx.clone()
}

/// 获取动态类型事件发送器
#[inline]
pub async fn get_dyn_sender(name: &str) -> Option<mpsc::UnboundedSender<Box<dyn Any + Send>>> {
    let channels = EVENT_CHANNELS.lock().await;
    channels.dyn_channels
        .iter()
        .find(|entry| entry.name == name)
        .map(|entry| entry.tx.clone())
}

/// 注册事件监听器
pub async fn register_event_listeners(
    string_listener: StringListener,
    dyn_listeners: &[DynListener]
) {
    let mut channels = EVENT_CHANNELS.lock().await;

    // 设置字符串事件通道
    let (string_tx, string_rx) = mpsc::unbounded_channel();
    channels.string_tx = string_tx;

    // 启动字符串事件监听器
    tokio::spawn(string_listener(string_rx));
    info!("String event listener spawned");

    // 清空旧的发送器
    channels.dyn_channels.clear();

    // 为每个监听器创建独立通道
    for (name, listener) in dyn_listeners {
        let (tx, rx) = mpsc::unbounded_channel();
        channels.dyn_channels.push(DynChannelEntry {
            name: name.clone(),
            tx,
        });
        tokio::spawn(listener(rx));
        info!("Dynamic event listener '{}' spawned", name);
    }
}

//*****************************************************************************
// 路由信息收集
//*****************************************************************************

#[derive(Clone, Debug)]
pub struct RouteInfo {
    pub path: String,
    pub method: Method,
    pub service_name: String,
    pub summary: String,
}

impl RouteInfo {
    pub fn new(path: &str, method: Method, service_name: &str, summary: &str) -> Self {
        RouteInfo {
            path: path.to_string(),
            method,
            service_name: service_name.to_string(),
            summary: summary.to_string(),
        }
    }
}
// 全局收集器
pub static ROUTE_COLLECTOR: Lazy<Mutex<Vec<RouteInfo>>> = Lazy::new(|| Mutex::new(Vec::new()));
// 添加路由
pub async fn add_route(route: RouteInfo) {
    ROUTE_COLLECTOR.lock().await.push(route);
}
// 获取路由
pub async fn get_collected_routes() -> Vec<RouteInfo> {
    ROUTE_COLLECTOR.lock().await.clone()
}
// 清除路由
pub async fn clear_routes() {
    ROUTE_COLLECTOR.lock().await.clear();
}

//*****************************************************************************
// 操作日志
//*****************************************************************************

#[derive(Debug, Clone)]
pub struct OperationLogContext {
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub domain: Option<String>,
    pub module_name: String,
    pub description: String,
    pub request_id: String,
    pub method: String,
    pub url: String,
    pub ip: String,
    pub user_agent: Option<String>,
    pub params: Option<Value>,
    pub body: Option<Value>,
    pub response: Option<Value>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub duration: i32,
    pub created_at: NaiveDateTime,
}

static OPERATION_LOG_CONTEXT: Lazy<Arc<RwLock<Option<OperationLogContext>>>> = Lazy::new(||
    Arc::new(RwLock::new(None))
);

impl OperationLogContext {
    pub async fn set(context: OperationLogContext) {
        let mut writer = OPERATION_LOG_CONTEXT.write().await;
        *writer = Some(context);
    }

    pub async fn get() -> Option<OperationLogContext> {
        OPERATION_LOG_CONTEXT.read().await.clone()
    }

    pub async fn clear() {
        let mut writer = OPERATION_LOG_CONTEXT.write().await;
        *writer = None;
    }
}

/// 异步发送字符串事件
#[inline]
pub fn send_string_event(msg: String) {
    tokio::spawn(async move {
        let sender = get_string_sender().await;
        let _ = sender.send(msg);
    });
}

/// 异步发送动态类型事件
#[inline]
pub fn send_dyn_event(event_name: &'static str, event: Box<dyn Any + Send>) {
    tokio::spawn(async move {
        if let Some(sender) = get_dyn_sender(event_name).await {
            let _ = sender.send(event);
        }
    });
}
