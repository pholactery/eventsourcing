use super::super::Result;
use super::EnrichedEvent;
use super::EventStore;
use chrono::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct MemoryEventStore<E>
where
    E: Serialize + Clone,
{
    evts: Mutex<Vec<EnrichedEvent<E>>>,
}

impl<E> MemoryEventStore<E>
where
    E: Serialize + Clone,
{
    pub fn new() -> MemoryEventStore<E> {
        MemoryEventStore {
            evts: Mutex::new(Vec::<EnrichedEvent<E>>::new()),
        }
    }
}

impl<E> EventStore<E> for MemoryEventStore<E>
where
    E: Serialize + Clone,
{
    fn append(&self, evt: E) -> Result<EnrichedEvent<E>> {
        let mut guard = self.evts.lock().unwrap();
        let enriched = EnrichedEvent {
            serial_number: "this-should-be-guid".to_owned(),
            version: 1,
            payload: evt,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        };
        guard.push(enriched.clone());
        Ok(enriched)
    }
}
