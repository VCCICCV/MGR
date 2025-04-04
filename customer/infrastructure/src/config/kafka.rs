use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct KafkaConfig {
    brokers: String,
}
impl KafkaConfig{
    pub fn get_brokers(&self) -> String {
        format!("{}", self.brokers)
    }
}