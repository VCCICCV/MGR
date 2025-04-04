// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::time::Duration;
//     use tokio::time::sleep;

//     #[tokio::test]
//     async fn test_mq_producer_and_consumer() {
//         // 创建生产者
//         let producer = MqProducer::new("localhost:9092").await.expect("Failed to create producer");
//         producer.send("test_topic", b"Hello from producer", None).await.expect("Failed to send message");

//         // 创建消费者
//         let consumer = MqConsumer::new("localhost:9092").await.expect("Failed to create consumer");
//         consumer.subscribe(&["test_topic"]).await.expect("Failed to subscribe to topic");

//         // 等待一段时间让消费者接收消息
//         sleep(Duration::from_secs(5)).await;

//         if let Some(message) = consumer.receive(Duration::from_secs(10)).await {
//             assert_eq!(message.payload(), b"Hello from producer");
//         } else {
//             panic!("No message received");
//         }
//     }
// }