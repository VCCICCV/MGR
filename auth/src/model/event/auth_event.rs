use serde::Serialize;
use uuid::Uuid;
use anyhow::Result;
// 建造者模式
#[derive(Default)]
pub struct Event {
    pub id: Uuid, // 必须通过builder设置
    pub source: String, // 有默认值"unknown"
    pub payload: String, // 必须通过builder设置
    pub event_type: String, // 必须通过builder设置
    pub version: i32, // 有默认值1
}
impl Event {
    pub fn builder() -> EventBuilder {
        EventBuilder::new()
    }
}
#[derive(Default)]
pub struct EventBuilder {
    inner: Event,
}

impl EventBuilder {
    pub fn new() -> Self {
        Self {
            inner: Event {
                source: "auth-service".to_string(), // 默认源
                version: 1, // 默认版本
                // 以下字段必须通过builder设置
                id: Uuid::nil(), // 临时值
                payload: String::new(), // 临时值
                event_type: String::new(), // 临时值
            },
        }
    }

    // 必须设置的字段
    pub fn id(mut self, id: Uuid) -> Self {
        self.inner.id = id;
        self
    }

    pub fn event_type(mut self, event_type: impl Into<String>) -> Self {
        self.inner.event_type = event_type.into();
        self
    }

    pub fn payload(mut self, payload: impl Serialize) -> Self {
        self.inner.payload = serde_json
            ::to_value(payload)
            .expect("Failed to serialize payload")
            .to_string();
        self
    }

    // 可选设置的字段（覆盖默认值）
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.inner.source = source.into();
        self
    }

    pub fn version(mut self, version: i32) -> Self {
        self.inner.version = version;
        self
    }

    pub fn build(self) -> Result<Event> {
        // 构建时校验必填字段
        if self.inner.id == Uuid::nil() {
            anyhow::bail!("Event id must be set");
        }
        if self.inner.event_type.is_empty() {
            anyhow::bail!("Event type must be set");
        }
        if self.inner.payload.is_empty() {
            anyhow::bail!("Event payload must be set");
        }

        Ok(self.inner)
    }
}
