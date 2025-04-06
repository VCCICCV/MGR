use rdkafka::{
    consumer::{ Consumer, StreamConsumer },
    producer::FutureProducer,
    ClientConfig,
    Message,
};
use anyhow::Result;
use tracing::info;
use crate::configure::AppConfig;

// 类型别名
pub type KafkaProducer = FutureProducer;
pub type KafkaConsumer = StreamConsumer;

// 客户端构建特征

pub trait KafkaClientBuilder {
    async fn from_config(config: &AppConfig) -> Result<Self> where Self: Sized;
}

// 生产者实现

impl KafkaClientBuilder for KafkaProducer {
    async fn from_config(config: &AppConfig) -> Result<Self> {
        let mut client_config = ClientConfig::new();
        client_config
            .set("bootstrap.servers", &config.kafka.get_brokers())
            .set("message.timeout.ms", "5000");

        let producer = client_config.create()?;
        info!("Kafka producer connected");
        Ok(producer)
    }
}

// 消费者实现
impl KafkaClientBuilder for KafkaConsumer {
    async fn from_config(config: &AppConfig) -> Result<Self> {
        let mut client_config = ClientConfig::new();
        client_config
            .set("bootstrap.servers", &config.kafka.get_brokers())
            .set("group.id", "my_consumer_group")
            .set("auto.offset.reset", "earliest");

        let consumer = client_config.create()?;
        info!("Kafka consumer connected");
        Ok(consumer)
    }
}

// 便捷发布接口
pub struct KafkaPublisher {
    producer: KafkaProducer,
}

impl KafkaPublisher {
    pub async fn new(config: &AppConfig) -> Result<Self> {
        Ok(Self {
            producer: KafkaProducer::from_config(config).await?,
        })
    }

    pub async fn publish(&self, topic: &str, key: &str, payload: &[u8]) -> Result<()> {
        let record = rdkafka::producer::FutureRecord::to(topic).key(key).payload(payload);

        self.producer.send(record, std::time::Duration::from_secs(5)).await;
        Ok(())
    }
}

// 便捷监听接口
pub struct KafkaSubscriber {
    consumer: KafkaConsumer,
}

impl KafkaSubscriber {
    pub async fn new(config: &AppConfig, topics: &[&str]) -> Result<Self> {
        let consumer = KafkaConsumer::from_config(config).await?;
        consumer.subscribe(topics)?;
        Ok(Self { consumer })
    }

    pub async fn listen<F>(&self, handler: F) -> Result<()>
        where F: Fn(&str, &[u8]) -> Result<()> + Send + Sync + 'static
    {
        loop {
            match self.consumer.recv().await {
                Ok(msg) => {
                    let topic = msg.topic();
                    if let Some(payload) = msg.payload() {
                        handler(topic, payload)?;
                    }
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }
}
