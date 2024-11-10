//! 基础设施层
//! 底层具体技术实现
// po持久化对象，有的的地方叫do，在rust中do是保留关键字，本质上是对数据表的映射
pub mod po {
    pub mod prelude;
    pub mod user;
}

/// 工具类
pub mod utils {
    pub mod random;
    pub mod redis_util;
    // pub mod jwt_util;
    pub mod password_util;
    pub mod task;
    pub mod dir;
}
/// 与表的映射实体
// pub mod entities{
//     // pub mod prelude;
//     // pub mod user;
// }
/// 持久层具体实现，对应表的CRUD
pub mod persistence {
    pub mod user_persistence;
    pub mod user_role_persistence;
}
// 持久层具体CRUD接口实现，对应domain的repository实现，这里用于对象转换和具体持久接口调用
pub mod repositories {
    pub mod customer_repository;
    pub mod auth_repository;
}
// RPC调用
pub mod remote {}
// 日志
pub mod logger {
    pub mod log;
}
// 聚合转DO，这里DO就是entity下与数据表对应的结构体
pub mod converter {
    pub mod customer_converter;
}

// 客户端
pub mod client {
    pub mod builder;
    pub mod database;
    pub mod redis;
    pub mod kafka;
    pub mod es;
    pub mod email;

}
// 全局配置
pub mod config;
pub mod constant;
pub mod mq {
    pub mod consumer;
    pub mod producer;
}
// state
pub mod state;
