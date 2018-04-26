use super::Result;
use chrono::prelude::*;
use serde::Serialize;
use std::collections::HashMap;

pub use self::inmemory::MemoryEventStore;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnrichedEvent<E>
where
    E: Clone + Serialize,
{
    serial_number: String,
    version: u32,
    payload: E,
    metadata: HashMap<String, String>,
    timestamp: DateTime<Utc>,
}

/// When an Event Store subscribes to events coming out of an aggregate,
/// they are wrapped/enriched for storage and retrieval
impl<E> EnrichedEvent<E>
where
    E: Clone + Serialize,
{
    pub fn new(payload: E, version: u32) -> EnrichedEvent<E> {
        EnrichedEvent {
            serial_number: "foo".to_owned(),
            version: version,
            payload: payload,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        }
    }
}

pub trait EventStore<E>
where
    E: Clone + Serialize,
{
    fn append(&self, evt: E) -> Result<EnrichedEvent<E>>;
}

mod inmemory;
