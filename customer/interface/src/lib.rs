//! 应用程序的入口点，用于启动应用程序
/// 负责与用户的交互，接收用户的请求，调用业务逻辑层的方法，返回响应
/// 这一层可以不单独拿出来放在application层
pub mod server;
/// 为什么要多加一层apapter？有的项目叫trigger（触发器）
/// CQRS职责分离，这里的目的是为了将业务逻辑和HTTP请求处理解耦，使应用程序的核心逻辑更加清晰，易于测试和维护
/// 将从业务逻辑层获取的数据转换为适合接口（例如 HTTP 响应）的格式
/// 只关注如何与外部进行交互，而不涉及具体的业务逻辑的实现细节
// restful api 适配器
pub mod api {
    pub mod openapi;
    pub mod server_handler;
    pub mod customer_handler;
    pub mod admin {
        pub mod user_handler;
    }
    pub mod token;
}
// 如果响应是grpc，那么就加一个grpc适配器，和infrastructure的grpc不同，这里是响应前端，infrasteucture的grpc是请求第三方服务
pub mod grpc {}
/// cmd
pub mod cmd {
    pub mod shutdown;
}
/// 路由
pub mod routers;

// 全局状态
pub mod state;
// 中间件
pub mod middleware {
    pub mod auth;
}