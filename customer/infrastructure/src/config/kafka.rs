// use serde::Deserialize;

// #[derive(Debug, Deserialize, Clone)]
// pub struct KafkaConfig {
//     brokers: String,
// }
// impl KafkaConfig{
//     pub fn get_brokers(&self) -> String {
//         format!("{}", self.brokers)
//     }
// }
// #[cfg(test)]
// mod tests {
//     use super::KafkaConfig;

//     #[test]
//     fn test_get_brokers() {
//         let config = KafkaConfig {
//             brokers: "127.0.0.1:9092".to_string(),
//         };
//         let brokers = config.get_brokers();
//         assert_eq!(brokers, "127.0.0.1:9092");
//     }
// }