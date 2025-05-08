# MGR

## 感谢

没有他们的指导就没有这个项目

* 感谢微信公众号“育儿之家YEZJ”作者对项目的指导
* 感谢“eventually.rs“作者Danilo Cianfrone对项目的指导
* 感谢“cassie_axum”作者对项目的指导

## 快速开始

安装sea-orm-cli

```cmd
cargo add seaorm-cli
```

运行迁移（迁移出现问题请删除target目录）

```cmd
sea-orm-cli migrate up
```

删除所有表

```cmd
# 不建议，状态不会删除
sea-orm-cli migrate down -n 30
# 删除定义的状态
DROP TYPE IF EXISTS menu_type;
DROP TYPE IF EXISTS status;
```

每次删除一个表

```cmd
sea-orm-cli migrate down
```

启动应用(重构后更适合分布式应用)

```cmd
cd server/bin

cargo run --bin user
cargo run --bin auth
cargo run --bin admin
cargo run --bin product
cargo run --bin order
```

生成实体
> 减少实体转换

```cmd
sea-orm-cli generate entity -o server/model/src/entities --with-serde both
```

快速开发

```cmd
cargo install cargo-watch

cargo watch -c -x 'run --bin admin'
```

## 架构

```bash
.
├── api                 # api handler层
├── bin                 # 二进制crate
├── codegen             # 代码生成器
├── config              # 配置文件解析
├── initialize          # 初始化层
├── middleware          # 中间件层
├── model               # 模型/实体层
├── router              # 路由层
├── service             # 业务层
├── settings            # 配置文件
├── shared              # 公共代码
└── static              # 静态资源
```

## 操作规约

| CRUD     | 命名约定 |
| -------- | ----- |
| 创建     | post|
| 删除     | delete|
| 查询单个 | get_by|
| 查询多个 | get_all|
| 分页查询 | get_page|
| 统计     | count |

## 问题？

为什么使用动态分发？

* 堆分配和动态分发会引入轻微性能损耗，但在复杂异步逻辑中通常是可接受的

为什么在全局懒加载配置后仍然将其他配置也写入全局？

* 最小知道原则（Law of Demeter, LoD）：​一个对象应当对其他对象保持最少的了解
* 当修改redis连接后，不希望postgres链接受影响

为什么将错误定义在每个模块？

* 大型项目采用模块级错误+全局包装错误
* 高内聚：新增功能无需修改全局错误
* 清晰语义

为什么不使用AppState？

* 每一层都需要传递，容易造成循环依赖，不希望像go一样到处传ctx
* 减少对框架的依赖

## 服务划分

* 用户服务
* 商品服务
* 订单服务
* 支付服务
* 通知服务
<!-- * CMS（Content Management System，内容管理系统）：商城所有可视化内容
* OMS（Order Management System，订单管理系统）​​：
* SYS（System Management，系统管理平台）​​：
* UMS（User Management System，用户管理系统）​​：认证、用户画像等 -->
## 解决方案

打开一片红则

```cmd
cargo clean

cargo build
```

## B2C&C2C（本项目采用B2C）

B2C（Business to Consumer）：企业直接面向消费者销售商品或服务

* 有自己的仓储管理

C2C（Consumer to Consumer）：消费者之间直接交易，平台仅提供撮合服务

* 没有精细化的仓储管理
