use config::AppConfig;
use rdkafka::{ ClientConfig, consumer::StreamConsumer, producer::FutureProducer };
use shared::error::AppError;
use tracing::info;


// 类型别名，方便后续使用，定义 Kafka 客户端相关的类型别名
pub type KafkaClientProducer = FutureProducer;
pub type KafkaClientConsumer = StreamConsumer;
// 定义一个特征，用于扩展 Kafka 客户端连接相关操作的通用行为
pub trait KafkaClientExt: Sized {
    fn build_from_config(
        config: &AppConfig
    ) -> impl std::future::Future<Output = Result<Self, AppError>>;
}
// 为 KafkaClientProducer 实现 KafkaClientExt 特征，用于创建生产者客户端连接
impl KafkaClientExt for KafkaClientProducer {
    async fn build_from_config(config: &AppConfig) -> Result<Self, AppError> {
        let mut producer_config = ClientConfig::new();
        producer_config.set("bootstrap.servers", &config.kafka.get_brokers());
        // 动态生成事务ID（确保唯一性）
        // let tx_id = format!("tx-{}-{}", "auth", Uuid::new_v4());
        // producer_config.set("transactional.id", &tx_id);
        producer_config.set("enable.idempotence", "true"); // 启用幂等性
        producer_config.set("acks", "all"); // 需要所有副本确认
        producer_config.set("linger.ms", "5"); // 适当批量发送
        let producer = producer_config.create()?;
        info!("Kafka producer client connected");
        Ok(producer)
    }
}

// 为 KafkaClientConsumer 实现 KafkaClientExt 特征，用于创建消费者客户端连接
impl KafkaClientExt for KafkaClientConsumer {
    async fn build_from_config(config: &AppConfig) -> Result<Self, AppError> {
        let mut consumer_config = ClientConfig::new();
        consumer_config.set("bootstrap.servers", &config.kafka.get_brokers());
        consumer_config.set("session.timeout.ms", "6000");
        consumer_config.set("enable.auto.commit", "false");
        consumer_config.set("auto.offset.reset", "earliest"); // 从最早的开始消费
        consumer_config.set("group.id", "auh-consumer"); // 关键配置
        // consumer_config.set("isolation.level", "read_committed"); // 关键隔离级别
        let consumer = consumer_config.create()?;
        info!("Kafka consumer client connected");
        Ok(consumer)
    }
}
// #[cfg(test)]
// mod tests {
//     use std::time::Duration;

//     use super::*;
//     use crate::constant::CONFIG;
//     use rdkafka::{
//         Message,
//         consumer::Consumer,
//         message::Headers,
//         producer::{FutureRecord, Producer},
//     };
//     use uuid::Uuid;
//     // 业务主题常量
//     const ORDER_TOPIC: &str = "orders";
//     const INVENTORY_TOPIC: &str = "inventory";
//     #[derive(serde::Serialize, serde::Deserialize, Debug)]
//     struct OrderEvent {
//         order_id: String,
//         user_id: String,
//         product_id: String,
//         quantity: i32,
//     }

//     #[derive(serde::Serialize, serde::Deserialize, Debug)]
//     struct InventoryEvent {
//         product_id: String,
//         deduct_quantity: i32,
//     }
//     #[tokio::test]
//     async fn test_order_transaction_flow() {
//         let config = CONFIG.clone();

//         // 创建事务生产者
//         let producer = KafkaClientProducer::build_from_config(&config)
//             .await
//             .expect("生产者创建失败");
//         producer
//             .init_transactions(Duration::from_secs(10))
//             .expect("事务初始化失败");

//         // 生成业务数据
//         let order_id = Uuid::new_v4().to_string();
//         let user_id = Uuid::new_v4().to_string();
//         let product_id = "PRODUCT_001".to_string();
//         let quantity = 2;

//         // 开启事务
//         producer.begin_transaction().expect("开启事务失败");

//         // 发送订单创建消息
//         let order_event = OrderEvent {
//             order_id: order_id.clone(),
//             user_id: user_id.clone(),
//             product_id: product_id.clone(),
//             quantity,
//         };
//         let order_payload = serde_json::to_vec(&order_event).unwrap();
//         let order_record = FutureRecord::to(ORDER_TOPIC)
//             .key(&order_id)
//             .payload(&order_payload);

//         let order_send_result = producer.send(order_record, Duration::from_secs(5)).await;
//         assert!(order_send_result.is_ok(), "订单消息发送失败");

//         // 发送库存扣减消息
//         let inventory_event = InventoryEvent {
//             product_id: product_id.clone(),
//             deduct_quantity: quantity,
//         };
//         let inventory_payload = serde_json::to_vec(&inventory_event).unwrap();
//         let inventory_record = FutureRecord::to(INVENTORY_TOPIC)
//             .key(&product_id)
//             .payload(&inventory_payload);

//         let inventory_send_result = producer
//             .send(inventory_record, Duration::from_secs(5))
//             .await;
//         assert!(inventory_send_result.is_ok(), "库存消息发送失败");

//         // 提交事务（实际业务中可在此处加入校验逻辑）
//         producer
//             .commit_transaction(Duration::from_secs(10))
//             .expect("事务提交失败");
//     }
//     #[tokio::test]
//     async fn test_order_transaction_consumer() {
//         let consumer = KafkaClientConsumer::build_from_config(&CONFIG)
//             .await
//             .expect("消费者创建失败");

//         // 订阅业务主题
//         consumer
//             .subscribe(&[ORDER_TOPIC, INVENTORY_TOPIC])
//             .expect("订阅主题失败");

//         let timeout = Duration::from_secs(30);
//         let start_time = std::time::Instant::now();
//         let mut received_orders = 0;
//         let mut received_inventory = 0;

//         loop {
//             if start_time.elapsed() > timeout {
//                 panic!("消费消息超时");
//             }

//             match tokio::time::timeout(Duration::from_secs(5), consumer.recv()).await {
//                 Ok(Ok(message)) => {
//                     let topic = message.topic();
//                     let payload = message.payload().unwrap();

//                     match topic {
//                         ORDER_TOPIC => {
//                             let order_event: OrderEvent = serde_json::from_slice(payload).unwrap();
//                             println!("收到订单事件: {:?}", order_event);
//                             received_orders += 1;
//                         }
//                         INVENTORY_TOPIC => {
//                             let inventory_event: InventoryEvent =
//                                 serde_json::from_slice(payload).unwrap();
//                             println!("收到库存事件: {:?}", inventory_event);
//                             received_inventory += 1;
//                         }
//                         _ => panic!("收到未知主题消息"),
//                     }

//                     // 两个事件都收到后退出
//                     if received_orders >= 1 && received_inventory >= 1 {
//                         return;
//                     }
//                 }
//                 Ok(Err(e)) => panic!("消费错误: {}", e),
//                 Err(_) => {
//                     continue;
//                 }
//             }
//         }
//     }
// #[tokio::test]
// async fn test_transactional_producer() {
//     let mut config = CONFIG.clone();
//     let producer = KafkaClientProducer::build_from_config(&config).await.expect(
//         "生产者创建失败"
//     );

//     // 初始化事务
//     producer.init_transactions(Duration::from_secs(10)).expect("事务初始化失败");

//     let topic = "test_topic";

//     // 开启事务
//     producer.begin_transaction().expect("开启事务失败");

//     // 发送事务消息
//     let delivery_status = producer.send(
//         FutureRecord::to(topic).payload("transactional_message").key("tx_key"),
//         Duration::from_secs(5)
//     ).await;

//     assert!(delivery_status.is_ok(), "消息发送失败");

//     // 提交事务
//     producer.commit_transaction(Duration::from_secs(10)).expect("事务提交失败");
// }

// #[tokio::test]
// async fn test_build_kafka_producer() {
//     let result = KafkaClientProducer::build_from_config(&CONFIG).await;
//     assert!(result.is_ok());
//     if let Ok(producer) = result {
//         // 可以在这里添加更多对 producer 的验证逻辑，比如尝试发送一个简单消息来确认连接可用等
//         // 以下只是简单示例，实际中可以根据 rdkafka 库的 API 进一步完善
//         let topic = "test_topic";
//         let delivery_status = producer.send::<Vec<u8>, _, _>(
//             rdkafka::producer::FutureRecord::to(topic).payload(b"test message"),
//             std::time::Duration::from_secs(0)
//         ).await;
//         assert!(delivery_status.is_ok());
//     }
// }

// #[tokio::test]
// async fn test_build_kafka_consumer() {
//     let result = KafkaClientConsumer::build_from_config(&CONFIG).await;
//     if let Ok(consumer) = result {
//         // 订阅一个测试主题
//         consumer.subscribe(&["test_topic"]).expect("订阅主题失败");
//         // 消费消息
//         loop {
//             match consumer.recv().await {
//                 Ok(message) => {
//                     if let Some(payload) = message.payload() {
//                         let text = String::from_utf8_lossy(payload);
//                         println!(
//                             "Received message: Topic: {}, Partition: {}, Offset: {}, Payload: {}",
//                             message.topic(),
//                             message.partition(),
//                             message.offset(),
//                             text
//                         );
//                     } else {
//                         println!("Received message with empty payload");
//                     }
//                 }
//                 Err(e) => panic!("消费消息失败: {}", e),
//             }
//         }
//     }
// }
// }
