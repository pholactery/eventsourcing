#![feature(attr_literals)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate eventsourcing_derive;
extern crate eventsourcing;
extern crate serde_json;

mod domain;

use domain::{Combat, CombatCommand, CombatDispatcher, CombatEvent, CombatState};
use eventsourcing::eventstore::OrgEventStore;
use eventsourcing::prelude::*;

fn main() {
    let combat_store = OrgEventStore::new("localhost", 2113);
    let swing = CombatCommand::Attack("ogre".to_owned(), 150);

    let state = CombatState {
        entity_id: "ogre".to_owned(),
        hitpoints: 900,
        generation: 0,
    };

    let rando = CombatEvent::RandomEvent { a: 12, b: 13 };
    println!("{}", rando.event_type());
    let unit = CombatEvent::UnitEvent;
    println!("{}", unit.event_type());

    let res = CombatDispatcher::dispatch(&state, swing, &combat_store, "ogre");
    println!("dispatch results - {:#?}", res);
}
