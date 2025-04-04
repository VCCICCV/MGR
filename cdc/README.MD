## 事务发件箱模式

<https://learn.microsoft.com/en-us/dotnet/architecture/microservices/multi-container-microservice-net-applications/subscribe-events#designing-atomicity-and-resiliency-when-publishing-to-the-event-bus>
流程：

* 应用开启本地数据库事务
* 更新领域实体（业务对象），将事件写入event表
* 提交事务
* 使用Debezium发布事件到Kafka
  * 写入后立即发布事件，并使用另一个本地事务将事件标记为已发布
  * 将表作为队列，使用单独的应用线程或进行查询事件表，将事件发布到事件总线，然后使用本地事务将事件标记为已发布
处理流程：
* 应用监听事件
* 将事件转换为命令
* 处理命令

## saga状态机
* STARTED：已启动
* SUCCEEDED：已成功
* ABORTING：正在中止
* ABORTED：已中止

## 事件表定义
下订单：
1. 存储领域实体，订单状态在支付后修改为已支付
2. 写事件表，存储支付订单对象的消息数据，事件状态未发送
3. debezium发送mq消息，搜到ack将消息更新为已发送

```
id payload status stepstate type version
```

* id：事件id
* payload：事件数据
* status：事件状态
* stepstate：事件步骤状态
* type：事件类型
* version：事件版本

## 补偿
* 人工补偿
* 定时任务补偿
* 手写反向任务补偿代码
## Debezium配置案例

```json5
{
    // Debezium连接器配置
    "connector.class": "io.debezium.connector.postgresql.PostgresConnector",
    // Debezium 连接器创建的最大任务数量
    "tasks.max": "1",
    // 要连接的数据库的主机名和端口
    "database.hostname": "postgres",
    "database.port": "5432",
    "database.user": "orderuser",
    "database.password": "orderpw",
    "database.dbname" : "postgres",
    // topic前缀
    "topic.prefix": "dbserver1",
    "schema.include.list": "purchaseorder",
    "table.include.list" : "purchaseorder.outboxevent",
    "tombstones.on.delete" : "false",
    // 转换为字符串格式
    "key.converter": "org.apache.kafka.connect.storage.StringConverter",
    "value.converter": "org.apache.kafka.connect.storage.StringConverter",
    // 启用了名为 “saga” 的数据转换操作
    "transforms" : "saga",
    // “saga” 这个转换操作的具体类型为EventRouter，它将事件路由到不同的Kafka主题
    "transforms.saga.type" : "io.debezium.transforms.outbox.EventRouter",
    //根据${routedByValue}这个占位符所代表的实际值（可能是事件中的某个字段值等）来构建新的消息主题，新主题的格式为 “<routedByValue>.request”
    "transforms.saga.route.topic.replacement" : "${routedByValue}.request",
    // Debezium 连接器对数据库进行轮询以获取变更事件的时间间隔，单位是毫秒
    "poll.interval.ms": "100"
}
```
