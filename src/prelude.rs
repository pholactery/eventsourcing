//! Standard prelude for eventsourcing applications
pub use super::{Aggregate, AggregateState, Event, Kind};


#[cfg(feature = "orgeventstore")]
pub use super::CloudEvent;
#[cfg(feature = "orgeventstore")]
pub use super::cloudevents::CloudEvent;
#[cfg(feature = "orgeventstore")]
pub use crate::eventstore::EventStore;
