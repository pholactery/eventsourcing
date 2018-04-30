use super::super::Event;
use super::super::Result;
use super::{EnrichedEvent, EventStore};
use std::sync::Mutex;

pub struct MemoryEventStore {
    evts: Mutex<Vec<EnrichedEvent>>,
}

impl MemoryEventStore {
    pub fn new() -> MemoryEventStore {
        MemoryEventStore {
            evts: Mutex::new(Vec::<EnrichedEvent>::new()),
        }
    }
}

impl EventStore for MemoryEventStore {
    fn append(&self, evt: impl Event) -> Result<EnrichedEvent> {
        let mut guard = self.evts.lock().unwrap();
        let enriched = EnrichedEvent::from(evt);
        guard.push(enriched.clone());
        Ok(enriched)
    }
}
