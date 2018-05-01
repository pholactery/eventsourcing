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

#[derive(Serialize, Deserialize, Debug, Clone)]
enum CombatEvent {
    EntityAttacked(String, u32),
}

impl Event for CombatEvent {
    fn schema_version(&self) -> u32 {
        1
    }

    fn event_type(&self) -> &str {
        match self {
            CombatEvent::EntityAttacked(_, _) => "combat.entity_attacked",
        }
    }
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

    let res = CombatDispatcher::dispatch(&state, swing, &combat_store);
    println!("dispatch results - {:#?}", res);
    println!(
        "store contents - {:#?}",
        combat_store.get_all("combat.entity_attacked")
    );
}
