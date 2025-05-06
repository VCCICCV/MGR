use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct KafkaConfig {
    pub brokers: String,
}
