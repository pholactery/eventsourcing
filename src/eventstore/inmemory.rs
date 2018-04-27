use super::super::Event;
use super::super::Result;
use super::{EnrichedEvent, EventStore};
use std::sync::Mutex;

pub struct MemoryEventStore<E>
where
    E: Event,
{
    evts: Mutex<Vec<EnrichedEvent<E>>>,
}

impl<E> MemoryEventStore<E>
where
    E: Event,
{
    pub fn new() -> MemoryEventStore<E> {
        MemoryEventStore {
            evts: Mutex::new(Vec::<EnrichedEvent<E>>::new()),
        }
    }
}

impl<E> EventStore<E> for MemoryEventStore<E>
where
    E: Event,
{
    fn append(&self, evt: E) -> Result<EnrichedEvent<E>> {
        let mut guard = self.evts.lock().unwrap();
        let enriched = EnrichedEvent::from(evt);
        guard.push(enriched.clone());
        Ok(enriched)
    }
}
