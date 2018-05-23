//! In-Memory Event Store
//!
//! This module provides an implementation of the event store trait for a simple in-memory
//! cache. This is not an event store you should be using for production and we recommend
//! it is recommended that you only use this for testing/demonstration purposes.
use super::super::cloudevents::CloudEvent;
use super::super::Event;
use super::super::Result;
use super::EventStore;
use chrono::prelude::*;
use std::sync::Mutex;

/// An simple, in-memory implementation of the event store trait
pub struct MemoryEventStore {
    evts: Mutex<Vec<CloudEvent>>,
}

impl MemoryEventStore {
    /// Creates a new in-memory event store. The resulting store is thread-safe.
    pub fn new() -> MemoryEventStore {
        MemoryEventStore {
            evts: Mutex::new(Vec::<CloudEvent>::new()),
        }
    }
}

impl EventStore for MemoryEventStore {
    /// Appends an event to the in-memory store
    fn append(&self, evt: impl Event, _stream: &str) -> Result<CloudEvent> {
        let mut guard = self.evts.lock().unwrap();
        let cloud_event = CloudEvent::from(evt);
        guard.push(cloud_event.clone());
        Ok(cloud_event)
    }
}

impl MemoryEventStore {
    pub fn get_all(&self, event_type: &str) -> Result<Vec<CloudEvent>> {
        let guard = self.evts.lock().unwrap();
        let matches = guard
            .iter()
            .filter(|evt| evt.event_type == event_type)
            .cloned()
            .collect();

        Ok(matches)
    }

    pub fn get_from(&self, event_type: &str, start: DateTime<Utc>) -> Result<Vec<CloudEvent>> {
        let guard = self.evts.lock().unwrap();
        let matches = guard
            .iter()
            .filter(|evt| evt.event_type == event_type && evt.event_time >= start)
            .cloned()
            .collect();
        Ok(matches)
    }

    pub fn get_range(
        &self,
        event_type: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<CloudEvent>> {
        let guard = self.evts.lock().unwrap();
        let matches = guard
            .iter()
            .filter(|evt| {
                evt.event_type == event_type && evt.event_time >= start && evt.event_time <= end
            })
            .cloned()
            .collect();
        Ok(matches)
    }
}
