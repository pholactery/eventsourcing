use eventsourcing::{
    Aggregate, AggregateState, Result,
};

#[derive(Debug)]
pub enum CombatCommand {
    Attack(String, u32),
}

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version("1.0")]
#[event_source("events://github.com/pholactery/eventsourcing/samples/combat")]
pub enum CombatEvent {
    EntityAttacked(String, u32),
    RandomEvent { a: u32, b: u32 },
    UnitEvent,
}

impl From<CombatCommand> for CombatEvent {
    fn from(source: CombatCommand) -> Self {
        match source {
            CombatCommand::Attack(entity_id, pts) => CombatEvent::EntityAttacked(entity_id, pts),
        }
    }
}

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
pub struct CombatDispatcher;
