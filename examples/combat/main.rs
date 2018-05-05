#![feature(attr_literals)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate eventsourcing_derive;
extern crate serde_json;
extern crate eventsourcing;

mod domain;

use eventsourcing::prelude::*;
use eventsourcing::eventstore::MemoryEventStore;
use domain::{CombatCommand, CombatDispatcher, CombatEvent, Combat, CombatState};

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
