#![feature(attr_literals)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate eventsourcing_derive;
extern crate eventsourcing;

use eventsourcing::{eventstore::{EnrichedEvent, EventStore, MemoryEventStore},
                    Aggregate,
                    AggregateState,
                    Dispatcher,
                    Event,
                    Result};

#[derive(Debug)]
enum CombatCommand {
    Attack(String, u32),
}

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[schema_version(1)]
enum CombatEvent {
    EntityAttacked(String, u32),
    RandomEvent { a: u32, b: u32},
    UnitEvent,
}

impl From<CombatCommand> for CombatEvent {
    fn from(source: CombatCommand) -> Self {
        match source {
            CombatCommand::Attack(entity_id, pts) => CombatEvent::EntityAttacked(entity_id, pts),
        }
    }
}

struct CombatState {
    entity_id: String,
    hitpoints: u32,
    generation: u64,
}

impl AggregateState for CombatState {
    fn generation(&self) -> u64 {
        self.generation
    }
}

struct Combat;
impl Aggregate for Combat {
    type Event = CombatEvent;
    type Command = CombatCommand;
    type State = CombatState;

    fn apply_event(state: &Self::State, evt: Self::Event) -> Result<Self::State> {
        unimplemented!()
    }

    fn handle_command(_state: &Self::State, cmd: Self::Command) -> Result<Vec<Self::Event>> {
        println!("Command handled: {:#?}", cmd);
        // SHOULD DO: validate state and command

        // if validation passes...
        Ok(vec![cmd.into()])
    }
}

#[derive(Dispatcher)]
#[aggregate(Combat)]
struct CombatDispatcher;

fn main() {
    let combat_store = MemoryEventStore::new();
    let swing = CombatCommand::Attack("ogre".to_owned(), 150);

    let state = CombatState {
        entity_id: "ogre".to_owned(),
        hitpoints: 900,
        generation: 0,
    };

    let rando = CombatEvent::RandomEvent { a: 12, b: 13};
    println!("{}", rando.event_type());
    let unit = CombatEvent::UnitEvent;
    println!("{}", unit.event_type());

    let res = CombatDispatcher::dispatch(&state, swing, &combat_store);
    println!("dispatch results - {:#?}", res);
    println!(
        "store contents - {:#?}",
        combat_store.get_all("combatevent.entityattacked")
    );
}
