//! Event store trait and implementations
use super::cloudevents::CloudEvent;
use super::{Event, Result};

pub use self::inmemory::MemoryEventStore;

#[cfg(feature = "orgeventstore")]
pub use self::orgeventstore::OrgEventStore;

/// Trait required for event stores. For the moment, event stores are append-only
pub trait EventStore {
    fn append(&self, evt: impl Event, stream: &str) -> Result<CloudEvent>;
}

mod inmemory;
#[cfg(feature = "orgeventstore")]
mod orgeventstore;
