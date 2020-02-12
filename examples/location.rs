extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate eventsourcing;
extern crate serde_json;
#[macro_use]
extern crate eventsourcing_derive;

use eventsourcing::{eventstore::MemoryEventStore, prelude::*, Result};

const DOMAIN_VERSION: &str = "1.0";

#[derive(Debug, Clone)]
struct LocationData {
    lat: f32,
    long: f32,
    alt: f32,
    generation: u64,
}

impl AggregateState for LocationData {
    fn generation(&self) -> u64 {
        self.generation
    }
}

#[event_type_version(DOMAIN_VERSION)]
#[event_source("events://github.com/pholactery/eventsourcing/samples/location")]
#[derive(Serialize, Deserialize, Debug, Clone, Event)]
enum LocationEvent {
    LocationUpdated { lat: f32, long: f32, alt: f32 },
}

struct Location;

enum LocationCommand {
    UpdateLocation { lat: f32, long: f32, alt: f32 },
}

impl Aggregate for Location {
    type Event = LocationEvent;
    type Command = LocationCommand;
    type State = LocationData;

    fn apply_event(state: &Self::State, evt: &Self::Event) -> Result<Self::State> {
        let ld = match *evt {
            LocationEvent::LocationUpdated { lat, long, alt } => LocationData {
                lat,
                long,
                alt,
                generation: state.generation + 1,
            },
        };
        Ok(ld)
    }

    fn handle_command(_state: &Self::State, cmd: &Self::Command) -> Result<Vec<Self::Event>> {
        // SHOULD DO: validate state and command
        let evt = match *cmd {
            LocationCommand::UpdateLocation { lat, long, alt } => {
                LocationEvent::LocationUpdated { lat, long, alt }
            }
        };

        // if validation passes...
        Ok(vec![evt])
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let location_store = MemoryEventStore::new();

    let update = LocationCommand::UpdateLocation {
        lat: 10.0,
        long: 52.0,
        alt: 31.0,
    };
    let old_state = LocationData {
        lat: 57.06,
        long: 36.07,
        alt: 15.0,
        generation: 0,
    };

    // First, handle a command to get an event vector
    let res = Location::handle_command(&old_state, &update)?;
    // Second, apply the events to get a new state
    let state = Location::apply_all(&old_state, &res)?;
    // Third, append to store (can do this alternatively with a dispatcher)
    let store_result = location_store.append(res[0].clone(), "locations")?;
    println!("Store result: {:?}", store_result);

    println!("Original state: {:?}", old_state);
    println!("Post-process state: {:?}", state);

    println!(
        "all events - {:#?}",
        location_store.get_all("locationevent.locationupdated")
    );

    Ok(())
}
