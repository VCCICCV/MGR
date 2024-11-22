//! 基础设施层
//! 底层具体技术实现
// po持久化对象，有的的地方叫do，在rust中do是保留关键字，本质上是对数据表的映射
pub mod po {
    pub mod prelude;
    pub mod user;
    pub mod receive_address;
}

/// 工具类
pub mod utils {
    pub mod dir;
    pub mod session;
    pub mod redis;
    pub mod token;
}
/// 与表的映射实体
// pub mod entities{
//     // pub mod prelude;
//     // pub mod user;
// }
/// 持久层具体实现，对应表的CRUD
pub mod persistence {
    pub mod customer_repository_impl;
}
// RPC调用
pub mod remote {}
// 聚合转DO，这里DO就是entity下与数据表对应的结构体
pub mod converter {
    pub mod customer_convert;
    pub mod user_convert;
}

// 客户端
pub mod client {
    pub mod builder;
    pub mod database;
    pub mod redis;
    pub mod es;
    pub mod email;
    pub mod kafka;
}
// 全局配置
pub mod config;
pub mod constant;
// 消息队列
pub mod event {
    pub mod consumer_impl;
    pub mod producer_impl;
}
// state
// pub mod state;
