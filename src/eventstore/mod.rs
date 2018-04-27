use super::{Event, Result};
use chrono::prelude::*;
use serde::Serialize;
use std::collections::HashMap;

pub use self::inmemory::MemoryEventStore;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnrichedEvent<E>
where
    E: Event,
{
    serial_number: String,
    schema_version: u32,
    payload: E,
    metadata: HashMap<String, String>,
    timestamp: DateTime<Utc>,
}

impl<E> From<E> for EnrichedEvent<E>
where
    E: Event,
{
    fn from(source: E) -> Self {
        EnrichedEvent {
            serial_number: "this-should-be-a-guid".to_owned(),
            schema_version: source.schema_version(),
            payload: source,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        }
    }
}

pub trait EventStore<E>
where
    E: Event,
{
    fn append(&self, evt: E) -> Result<EnrichedEvent<E>>;
}

mod inmemory;
