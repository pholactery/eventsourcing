use super::{Event, Result};
use chrono::prelude::*;
use serde_json;
use uuid::Uuid;

pub use self::inmemory::MemoryEventStore;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnrichedEvent {
    pub serial_number: String,
    pub event_type: String,
    pub schema_version: u32,
    pub payload: String,
    pub timestamp: DateTime<Utc>,
}

impl<E> From<E> for EnrichedEvent
where
    E: Event,
{
    fn from(source: E) -> Self {
        EnrichedEvent {
            serial_number: Uuid::new_v4().hyphenated().to_string(),
            event_type: source.event_type().to_owned(),
            schema_version: source.schema_version(),
            payload: serde_json::to_string(&source).unwrap(),
            timestamp: Utc::now(),
        }
    }
}
pub trait EventStore {
    fn append(&self, evt: impl Event) -> Result<EnrichedEvent>;
    fn get_all(&self, event_type: &str) -> Result<Vec<EnrichedEvent>>;
    fn get_from(&self, event_type: &str, start: DateTime<Utc>) -> Result<Vec<EnrichedEvent>>;
    fn get_range(
        &self,
        event_type: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<EnrichedEvent>>;
}

mod inmemory;
