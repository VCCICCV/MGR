use rdkafka::{ consumer::StreamConsumer, producer::FutureProducer, ClientConfig };
use tracing::info;
use anyhow::Result;

use crate::configure::AppConfig;

// 类型别名，方便后续使用，定义 Kafka 客户端相关的类型别名
pub type KafkaClientProducer = FutureProducer;
pub type KafkaClientConsumer = StreamConsumer;
// 定义一个特征，用于扩展 Kafka 客户端连接相关操作的通用行为
pub trait KafkaClientExt: Sized {
    fn build_from_config(config: &AppConfig) -> impl std::future::Future<Output = Result<Self>>;
}
// 为 KafkaClientProducer 实现 KafkaClientExt 特征，用于创建生产者客户端连接
impl KafkaClientExt for KafkaClientProducer {
    async fn build_from_config(config: &AppConfig) -> Result<Self> {
        let mut producer_config = ClientConfig::new();
        producer_config.set("bootstrap.servers", &config.kafka.get_brokers());
        producer_config.set("message.timeout.ms", "5000");
        let producer = producer_config.create()?;
        info!("Kafka producer client connected");
        Ok(producer)
    }
}

// 为 KafkaClientConsumer 实现 KafkaClientExt 特征，用于创建消费者客户端连接
impl KafkaClientExt for KafkaClientConsumer {
    async fn build_from_config(config: &AppConfig) -> Result<Self> {
        let mut consumer_config = ClientConfig::new();
        consumer_config.set("bootstrap.servers", &config.kafka.get_brokers());
        consumer_config.set("session.timeout.ms", "6000");
        consumer_config.set("enable.auto.commit", "false");
        consumer_config.set("auto.offset.reset", "earliest");
        consumer_config.set("group.id", "rust-rdkafka-smol-runtime-example");
        let consumer = consumer_config.create()?;
        info!("Kafka consumer client connected");
        Ok(consumer)
    }
}
#[cfg(test)]
mod tests {
    use rdkafka::{ consumer::Consumer, Message };
    use super::*;
    use crate::constant::CONFIG;
    #[tokio::test]
    async fn test_build_kafka_producer() {
        let result = KafkaClientProducer::build_from_config(&CONFIG).await;
        assert!(result.is_ok());
        if let Ok(producer) = result {
            // 可以在这里添加更多对 producer 的验证逻辑，比如尝试发送一个简单消息来确认连接可用等
            // 以下只是简单示例，实际中可以根据 rdkafka 库的 API 进一步完善
            let topic = "test_topic";
            let delivery_status = producer.send::<Vec<u8>, _, _>(
                rdkafka::producer::FutureRecord::to(topic).payload(b"test message"),
                std::time::Duration::from_secs(0)
            ).await;
            assert!(delivery_status.is_ok());
        }
    }

    #[tokio::test]
    async fn test_build_kafka_consumer() {
        let result = KafkaClientConsumer::build_from_config(&CONFIG).await;
        if let Ok(consumer) = result {
            // 订阅一个测试主题
            consumer.subscribe(&["test_topic"]).expect("订阅主题失败");
            // 消费消息
            loop {
                match consumer.recv().await {
                    Ok(message) => {
                        if let Some(payload) = message.payload() {
                            let text = String::from_utf8_lossy(payload);
                            println!(
                                "Received message: Topic: {}, Partition: {}, Offset: {}, Payload: {}",
                                message.topic(),
                                message.partition(),
                                message.offset(),
                                text
                            );
                        } else {
                            println!("Received message with empty payload");
                        }
                    }
                    Err(e) => panic!("消费消息失败: {}", e),
                }
            }
        }
    }
}
