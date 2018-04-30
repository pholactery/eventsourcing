use super::{Event, Result};
use chrono::prelude::*;
use serde_json;
use uuid::Uuid;

pub use self::inmemory::MemoryEventStore;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnrichedEvent {
    serial_number: String,
    schema_version: u32,
    payload: String,
    timestamp: DateTime<Utc>,
}

impl<E> From<E> for EnrichedEvent
where
    E: Event,
{
    fn from(source: E) -> Self {
        EnrichedEvent {
            serial_number: Uuid::new_v4().hyphenated().to_string(),
            schema_version: source.schema_version(),
            payload: serde_json::to_string(&source).unwrap(),
            timestamp: Utc::now(),
        }
    }
}
pub trait EventStore {
    fn append(&self, evt: impl Event) -> Result<EnrichedEvent>;
}

mod inmemory;
