extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

use eventstore::EnrichedEvent;
use eventstore::EventStore;
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "An eventsourcing error ocurred"
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            Kind::ApplicationFailure(ref s) => fmt::Display::fmt(s, f),
            Kind::CommandFailure(ref s) => fmt::Display::fmt(s, f),
        }
    }
}

// this is useless at the moment, might have more value when event store failures
// can happen
#[derive(Debug)]
pub enum Kind {
    ApplicationFailure(String),
    CommandFailure(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Event: Serialize {
    fn schema_version(&self) -> u32;
    fn event_type(&self) -> &str;
}

pub trait AggregateState {
    fn generation(&self) -> u64;
}

pub trait Aggregate {
    type Event: Event;
    type Command;
    type State: AggregateState;

    fn apply_event(state: &Self::State, evt: Self::Event) -> Result<Self::State>;
    fn handle_command(state: &Self::State, cmd: Self::Command) -> Result<Vec<Self::Event>>;
}

pub trait Dispatcher {
    type Command;
    type Event: Event;
    type State: AggregateState;
    type Aggregate: Aggregate<Event = Self::Event, Command = Self::Command, State = Self::State>;

    fn dispatch(
        state: &Self::State,
        cmd: Self::Command,
        store: &impl EventStore,
    ) -> Vec<Result<EnrichedEvent>>;
}

pub mod eventstore;
pub mod prelude;