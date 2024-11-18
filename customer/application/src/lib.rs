//! 应用层
///
/// 协调各层的具体业务流程，不应该实现具体的业务逻辑，只应该调用下层的接口
/// 例如订单服务需要调用用户服务，订单服务应该调用用户服务的接口，而不是直接调用用户服务的具体实现
/// 通过关联函数注入下层的接口，这样可以方便地替换下层的实现

/// usecase就是能做的事情，这里不做CQRS，主要是为了简单，如果系统足够复杂，可以考虑分离职责
/// 一个用例一般是一个业务流程
/// 例如创建订单，创建订单需要调用用户服务的接口->调用商品服务的接口->调用订单服务的接口
/// excute执行具体的domain能力
/// 这一层可以放在shared中
/// 入参：命令（返回DTO或Bool）、查询对象（返回DTO或Collection）、事件对象（无返回值）
pub mod dto {
    // 查询
    pub mod query;
    // 命令
    pub mod command;
}
// 在这里注入依赖，通过调用领域服务的接口来完成具体的业务逻辑

// CQE规范：用例入参只有command、query、event
// CQE对象需要能代表当前方法的语意
// DTO是贫血对象，CQE是充血对象有明确的意图，即使有相同字段也应该避免复用，否则会导致混乱
// 每个方法处理一个用例；针对复杂业务流程可以增加command_handler、event_handler来降低代码量；
// 如何判断是编排流程而不是业务：1、不要有if/else（决策） 2、不要有计算 3、不要有对象转换的逻辑，但是可以有对象转换的方法调用（对象转换的逻辑放到Assmbler中）
// 用例做什么：1、准备数据（对象转换）2、调用领域服务domain service和领域repository编排业务逻辑和持久化
pub mod use_case {
    pub mod customer_use_case;
}
// // 将entity转换为dto
pub mod assembler {
    pub mod customer_assembler;
}
pub mod state;