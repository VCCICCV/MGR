//! 领域层
/// 领域模型是对现实世界的抽象表示，涵盖了整个业务中的所有概念、实体、值对象、服务和他们之间的关系
/// 领域模型中的服务通常是无状态的，它们可以被多个不同的客户端调用，执行特定的业务逻辑
/// 如，领域模型可能包含用户、商品、订单、购物车、支付方式和各种实体及他们之间的复杂关系
// pub mod model {
// }
/// 聚合：聚合是由多个实体和值对象组成的，聚合的根实体负责协调聚合内的实体之间的关系，聚合的根实体是聚合内的唯一标识
/// 聚合内部的实体和值对象只能通过聚合根进行访问，外部对象只能通过聚合根来操作聚合内部的元素
/// 对聚合的操作通常在一个事务中完成，以保证数据的一致性
/// 订单可以作为一个聚合。订单由订单头（包含订单编号、客户信息、订单状态等）和订单行项（包含商品信息、数量、价格等）组成
/// 订单头是聚合根，它负责管理订单行项的生命周期
/// 外部对象只能通过订单头来操作订单行项，例如添加、删除或修改订单行项
pub mod model {
    // // 值对象
    // pub mod vo{

    //     // pub mod user_password;
    // }
    // 领域原语：领域层中定义的一些基本类型，比如用户ID、商品ID等，将隐形的概念显性化
    // 主要用于参数校验
    // 与值对象相比增加了数据校验和行为
    // 值对象是一个非实体的概念，它不具有唯一标识，只由其属性的值定义
    // 将隐性的概念显性化
    // 将隐性的概念显性化
    // 封装 多对象 行为
    pub mod dp {
        pub mod customer_id;
        pub mod role;
    }
    //  聚合对象，由值对象和领域实体组成
    // 引起状态变化的方法放到充血模型
    pub mod aggregate {
        pub mod customer;
        pub mod order;
        pub mod receiver;
    }
    // 领域实体，有的项目叫BO业务对象
    pub mod entity {
        pub mod receive_address;
        // pub mod permission;
        // pub mod role_permission;
        // pub mod role;
    }
    pub mod reponse {
        pub mod response;
        pub mod error;
    }
    /// 入参：命令（返回DTO或Bool）、查询对象（返回DTO或Collection）、事件对象（无返回值）
    /// 如何区分一个请求是查询还是命令
    ///
    /// 查询：请求不会对系统状态产生修改
    /// 命令：请求会对系统状态产生修改
    pub mod dto {
        // 系统需要的传输对象，如写入redis的对象
        pub mod info;
    }
}
/// 高层Domain不应该依赖于低层Infrastructure，而是应该依赖于抽象trait
/// 在COLA架构中，这里叫gateway
/// 领域仓储只负责领域对象的持久化和查询
pub mod repositories {
    // 用户接口，抽象trait，在基础设施层中实现
    pub mod customer_repository;
}

/// 领域服务（领域能力）：这个领域提供的能力
/// 领域服务是领域层的核心，它应该是无状态的，并且不应该依赖于任何其他领域层的组件，应该通过repository来获取数据
/// 领域服务通过repository获取数据，只有业务无法归属于某个实体，那么这个能力就是一个领域服务
/// 尽量避免领域服务之间的调用，应该通过repsitory提供能力
/// 传入的参数应该是实体而不是单个参数，多对象操作通过领域服务实现
pub mod service {
    pub mod customer_service;
    pub mod customer_service_impl;
}
/// 值对象：没有唯一标识的对象，由其属性的值定义，通常是不可变的
/// 比如，地址可以作为一个值对象。地址由国家、省份、城市、街道、邮编等属性组成，这些属性的值共同定义了一个地址；数据校验可以作为值对象
/// 如果两个地址的所有属性的值都相同，那么这两个地址就是相等的

/// 定义事件总线的trait，表示发生的事件，在infrastructure层中实现
/// 通过发布订阅（观察者模式）模式，解决业务之间的耦合，订阅者之间互不认识也不干扰
/// 订单创建成功后，发布一个订单注册成功事件，订阅者根据需要订阅订单组测成功事件，编写相对应的处理程序
///
/// 事件总线类型
/// * 跨进程事件总线（集成事件总线）：发布者与订阅者不在同一个进程中，订阅者是一个新的请求
/// * 跨服务事件总线（消息队列）：发布者与订阅者不在同一个服务中，订阅者是一个新的请求
/// * 进程内事件总线（领域事件总线）：发布者与订阅者在同一个进程中，订阅者是出错会引起当前请求出错
pub mod event {
    // 定义事件对象
    pub mod email;
    pub mod consumer;
}
// 常量，像token需要用到加密的可以考虑将token抽象然后再基础设施实现
pub mod constant;
pub mod utils {
    pub mod redis;
    pub mod random;
    pub mod session;
    pub mod token;
    pub mod password;
    pub mod hash;
    pub mod claim;
}
