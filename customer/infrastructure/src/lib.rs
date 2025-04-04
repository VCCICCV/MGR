//! 基础设施层
//! 底层具体技术实现
// po持久化对象，有的的地方叫do，在rust中do是保留关键字，本质上是对数据表的映射
pub mod po;
/// 工具类
pub mod utils {
    pub mod dir;
    pub mod random;
    pub mod hash;
    pub mod session;
}

/// 与表的映射实体
// pub mod entities{
//     // pub mod prelude;
//     // pub mod user;
// }
/// 持久层具体实现，对应表的CRUD
/// 在COLA架构中，这里叫gatewayimpl
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
    // pub mod consumer_impl;
    // pub mod producer_impl;
}

pub mod domain {
    pub mod utils {
        pub mod redis_impl;
        pub mod token_impl;
        pub mod session_impl;
    }
}
