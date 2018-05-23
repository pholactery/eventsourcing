# Event Sourcing

An eventsourcing library for Rust

One of the benefits of [event sourcing](https://martinfowler.com/eaaDev/EventSourcing.html)
is that in most cases, embracing this pattern does not require that much code.
However, there's still a bit of boilerplate required as well as the discipline for ensuring
the events, commands, and aggregates all perform their roles without sharing concerns.

The fundamental workflow to remember is that **commands** are applied to **aggregates**,
which then emit one or more events. An **aggregate**'s business logic is also responsible
for returning a new state from a previous state combined with a new event. Put
mathematically, this looks like:

 ```terminal
 f(state1, event) = state2
 ```

There are some event sourcing libraries that allow for, or even encourage, mutation of
state in memory. I prefer a more functional approach, and the design of this library
reflects that. It encourages you to write unit tests for your aggregate business logic that
are predictable and can be executed in isolation without concern for how you receive events
or how you persist them in a store.
 
To start, you create an undecorated enum for your **Command** type:
 ```rust
 enum LocationCommand {
    UpdateLocation { lat: f32, long: f32, alt: f32 },
}
 ```

 Next, you create an enum for your events and use a derive macro to write some boilerplate
 on your behalf. Note how the command variants are _imperative_ statements while the
 event variants are _verbs phrases in the past tense_. While this is by convention and
 not enforced via code, this is a good practice to adopt.
 
 ```rust
#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version("1.0")]
#[event_source("events://github.com/pholactery/eventsourcing/samples/location")]
enum LocationEvent {
    LocationUpdated { lat: f32, long: f32, alt: f32 },
}
 ```

 We then define a type that represents the state to be used by an aggregate.
 With that in place, we write all of our business logic, the core of our event sourcing system,
 in the aggregate.
 
```rust
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

struct Location;
impl Aggregate for Location {
   type Event = LocationEvent;
   type Command = LocationCommand;
   type State = LocationData;

   fn apply_event(state: &Self::State, evt: Self::Event) -> Result<Self::State> {
       // TODO: validate event
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

   fn handle_command(_state: &Self::State, cmd: Self::Command) -> Result<Vec<Self::Event>> {
       // TODO: add code to validate state and command

       // if validation passes...
       Ok(vec![LocationEvent::LocationUpdated { lat: 10.0, long: 10.0, alt: 10.0 }])
   }
}
 ```
 
 For more examples of usage, check out the [examples](https://github.com/pholactery/eventsourcing/tree/master/examples) directory in github.