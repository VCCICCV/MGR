// use rdkafka::{
//     producer::{FutureProducer, FutureRecord},
//     error::KafkaError,
// };

// pub struct MqProducer {
//     producer: FutureProducer,
// }

// impl MqProducer {
//     pub fn new(brokers: &str) -> Result<Self, KafkaError> {
//         let producer: FutureProducer = rdkafka::ClientConfig::new()
//            .set("bootstrap.servers", brokers)
//            .create()?;
//         Ok(Self { producer })
//     }

//     pub fn send(&self, topic: &str, payload: &[u8], key: Option<&str>) -> Result<(), KafkaError> {
//         let record = FutureRecord::to(topic)
//            .payload(payload)
//            .key(key.unwrap_or(""));
//         self.producer.send(record, Duration::from_secs(0))?;
//         Ok(())
//     }
// }