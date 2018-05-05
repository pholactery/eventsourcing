use super::super::Event;
use super::super::Result;
use super::{EventStore};
use super::super::cloudevents::CloudEvent;
use chrono::prelude::*;
use std::sync::Mutex;
use serde::Serialize;

pub struct MemoryEventStore {
    evts: Mutex<Vec<CloudEvent>>,
}

impl MemoryEventStore {
    pub fn new() -> MemoryEventStore {
        MemoryEventStore {
            evts: Mutex::new(Vec::<CloudEvent>::new()),
        }
    }
}

impl EventStore for MemoryEventStore
{
    fn append(&self, evt: impl Event) -> Result<CloudEvent> {
        let mut guard = self.evts.lock().unwrap();
        let cloud_event = CloudEvent::from(evt);
        guard.push(cloud_event.clone());
        Ok(cloud_event)
    }

    fn get_all(&self, event_type: &str) -> Result<Vec<CloudEvent>> {
        let guard = self.evts.lock().unwrap();
        let matches = guard
            .iter()
            .filter(|evt| evt.event_type == event_type)
            .cloned()
            .collect();

        Ok(matches)
    }

    fn get_from(&self, event_type: &str, start: DateTime<Utc>) -> Result<Vec<CloudEvent>> {
        let guard = self.evts.lock().unwrap();
        let matches = guard
            .iter()
            .filter(|evt| evt.event_type == event_type && evt.event_time >= start)
            .cloned()
            .collect();
        Ok(matches)
    }

    fn get_range(
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
