const DOMAIN_VERSION: &str = "1.0";

use eventsourcing::{Aggregate, AggregateState, Result};

#[derive(Debug)]
pub enum CombatCommand {
    Attack(String, u32),
}

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version(DOMAIN_VERSION)]
#[event_source("events://github.com/pholactery/eventsourcing/samples/combat")]
pub enum CombatEvent {
    EntityAttacked(String, u32),
    RandomEvent { a: u32, b: u32 },
    UnitEvent,
}

#[derive(Debug, Clone)]
pub struct CombatState {
    pub entity_id: String,
    pub hitpoints: u32,
    pub generation: u64,
}

impl AggregateState for CombatState {
    fn generation(&self) -> u64 {
        self.generation
    }
}

pub struct Combat;
impl Aggregate for Combat {
    type Event = CombatEvent;
    type Command = CombatCommand;
    type State = CombatState;

    fn apply_event(_state: &Self::State, _evt: &Self::Event) -> Result<Self::State> {
        unimplemented!()
    }

    fn handle_command(_state: &Self::State, cmd: &Self::Command) -> Result<Vec<Self::Event>> {
        println!("Command handled: {:#?}", cmd);
        // SHOULD DO: validate state and command
        let evt = match *cmd {
            CombatCommand::Attack(ref entity_id, pts) => {
                CombatEvent::EntityAttacked(entity_id.clone(), pts)
            }
        };

        // if validation passes...
        Ok(vec![evt])
    }
}

#[derive(Dispatcher)]
#[aggregate(Combat)]
pub struct CombatDispatcher;
