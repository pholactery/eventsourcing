//! Event store trait and implementations

use super::{Event, Result};

pub use self::inmemory::MemoryEventStore;

#[cfg(feature = "orgeventstore")]
pub use self::orgeventstore::OrgEventStore;

/// Trait required for event stores. For the moment, event stores are append-only
pub trait EventStore<T> {
    fn append(&self, evt: impl Event, stream: &str) -> Result<T>;
}

mod inmemory;
#[cfg(feature = "orgeventstore")]
mod orgeventstore;
