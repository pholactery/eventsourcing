//! Standard prelude for eventsourcing applications
pub use super::cloudevents::CloudEvent;
pub use super::{Aggregate, AggregateState, Dispatcher, Event, Kind};
pub use crate::eventstore::EventStore;
