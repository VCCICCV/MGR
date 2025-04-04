//! 应用层
///这一层可以做e2e，查询可以直接用raw sql
///
///
/// 协调各层的具体业务流程，不应该实现具体的业务逻辑，只应该调用下层的接口
/// 例如订单服务需要调用用户服务，订单服务应该调用用户服务的接口，而不是直接调用用户服务的具体实现
/// 通过关联函数注入下层的接口，这样可以方便地替换下层的实现

/// 一个用例一般是一个业务流程
/// 例如创建订单，创建订单需要调用用户服务的接口->调用商品服务的接口->调用订单服务的接口
/// excute执行具体的domain能力
/// 这一层可以放在shared中

// 在这里注入依赖，通过调用领域服务的接口来完成具体的业务逻辑

// CQE规范：用例入参只有command、query、event
// CQE对象需要能代表当前方法的语意
// DTO是贫血对象，CQE是充血对象有明确的意图，即使有相同字段也应该避免复用，否则会导致混乱
// 每个方法处理一个用例；针对复杂业务流程可以增加command_handler、event_handler来降低代码量；
// 用例做什么：1、准备数据（对象转换）2、调用领域服务domain service和领域repository编排业务逻辑和持久化
// 这里的用例没有分为命令和查询是为了简单，如果用例比较复杂，可以考虑拆分
// 所有命令转bo，再调用领域服务
pub mod executor {
    pub mod customer_use_case;
    pub mod customer_use_case_impl;
}
// 装配器，组装领域对象
pub mod assembler {
    // pub mod customer_assembler;
}


pub mod dto {
    pub mod command;
    pub mod query;
}
// 查询模型
pub mod query_model {
    pub mod user;
}
