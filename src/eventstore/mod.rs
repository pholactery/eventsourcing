use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnrichedEvent<E> {
    serial_number: String,
    version: u32,
    payload: E,
    metadata: HashMap<String, String>,
    timestamp: DateTime<Utc>,
}

/// When an Event Store subscribes to events coming out of an aggregate,
/// they are wrapped/enriched for storage and retrieval
impl<E> EnrichedEvent<E> {
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
