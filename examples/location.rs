extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate eventsourcing;

use eventsourcing::{Aggregate, AggregateState, Result};

#[derive(Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

    fn apply_event(state: &Self::State, evt: Self::Event) -> Result<Self::State> {
        let ld = match evt {
            LocationEvent::LocationUpdated { lat, long, alt } => LocationData {
                lat,
                long,
                alt,
                generation: state.generation + 1,
            },
        };
        Ok(ld)
    }

    fn handle_command(_state: &Self::State, _cmd: Self::Command) -> Result<Vec<Self::Event>> {
        unimplemented!()
    }
}

fn main() {
    let _update = LocationCommand::UpdateLocation {
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

    let evt = LocationEvent::LocationUpdated {
        lat: 30.0,
        long: 20.0,
        alt: 35.0,
    };
    let state = Location::apply_event(&old_state, evt).unwrap();

    println!("{:#?}", state);
}
