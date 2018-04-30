use super::super::Event;
use super::super::Result;
use super::{EnrichedEvent, EventStore};
use chrono::prelude::*;
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

    fn get_all(&self, event_type: &str) -> Result<Vec<EnrichedEvent>> {
        let guard = self.evts.lock().unwrap();
        let matches = guard
            .iter()
            .filter(|evt| evt.event_type == event_type)
            .cloned()
            .collect();

        Ok(matches)
    }

    fn get_from(&self, event_type: &str, start: DateTime<Utc>) -> Result<Vec<EnrichedEvent>> {
        let guard = self.evts.lock().unwrap();
        let matches = guard
            .iter()
            .filter(|evt| evt.event_type == event_type && evt.timestamp >= start)
            .cloned()
            .collect();
        Ok(matches)
    }

    fn get_range(
        &self,
        event_type: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<EnrichedEvent>> {
        let guard = self.evts.lock().unwrap();
        let matches = guard
            .iter()
            .filter(|evt| {
                evt.event_type == event_type && evt.timestamp >= start && evt.timestamp <= end
            })
            .cloned()
            .collect();
        Ok(matches)
    }
}
