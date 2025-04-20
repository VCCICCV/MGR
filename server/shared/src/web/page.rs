use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageRequest {
    #[serde(
        default = "default_current",
        deserialize_with = "deserialize_u64_from_string"
    )]
    pub current: u64,
    #[serde(
        default = "default_size",
        deserialize_with = "deserialize_u64_from_string"
    )]
    pub size: u64,
}

fn deserialize_u64_from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<u64>().map_err(DeError::custom)
}

fn default_current() -> u64 {
    1
}

fn default_size() -> u64 {
    10
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            current: default_current(),
            size: default_size(),
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct PaginatedData<T> {
    pub current: u64,
    pub size: u64,
    pub total: u64,
    pub records: Vec<T>,
}
