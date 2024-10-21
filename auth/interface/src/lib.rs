//! 应用程序的入口点，用于启动应用程序
/// 负责与用户的交互，接收用户的请求，调用业务逻辑层的方法，返回响应
/// 这一层可以不单独拿出来放在application层
pub mod api{
    pub mod api;
}
/// 为什么要多加一层apapter？有的项目叫trigger（触发器）
/// CQRS职责分离，这里的目的是为了将业务逻辑和HTTP请求处理解耦，使应用程序的核心逻辑更加清晰，易于测试和维护
/// 将从业务逻辑层获取的数据转换为适合接口（例如 HTTP 响应）的格式
/// 只关注如何与外部进行交互，而不涉及具体的业务逻辑的实现细节
pub mod adapter{
    pub mod user_handler;
    // pub mod auth_handler;
}
/// 公共响应，用于封装应用程序的响应数据
/// 为什么不放在common中？
/// 我们希望在handler中将应用层和路由解耦，应用层和基础设施层只进行错误处理，响应码和信息在handler统一进行返回
// pub mod common{
//     pub mod response;
// }
/// 路由
pub mod routers{
    pub mod user_routes;
}

// state
// pub mod state;