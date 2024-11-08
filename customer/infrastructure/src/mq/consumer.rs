// use rdkafka::{
//     consumer::{Consumer, StreamConsumer},
//     error::KafkaError,
// };
// use std::time::Duration;

// pub struct MqConsumer {
//     consumer: StreamConsumer,
// }

// impl MqConsumer {
//     pub fn new(brokers: &str) -> Result<Self, KafkaError> {
//         let consumer: StreamConsumer = rdkafka::ClientConfig::new()
//            .set("bootstrap.servers", brokers)
//            .set("group.id", "my_group")
//            .create()?;
//         Ok(Self { consumer })
//     }

//     pub fn subscribe(&self, topics: &[&str]) -> Result<(), KafkaError> {
//         self.consumer.subscribe(topics)?;
//         Ok(())
//     }

//     pub fn receive(&self, timeout: Duration) -> Option<rdkafka::Message> {
//         self.consumer.recv()
//     }
// }