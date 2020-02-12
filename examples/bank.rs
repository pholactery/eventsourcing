extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate eventsourcing;
extern crate serde_json;
#[macro_use]
extern crate eventsourcing_derive;

use eventsourcing::{eventstore::MemoryEventStore, prelude::*, Result};

const DOMAIN_VERSION: &str = "1.0";

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version(DOMAIN_VERSION)]
#[event_source("events://github.com/pholactery/eventsourcing/samples/bank")]
enum BankEvent {
    FundsWithdrawn(String, u32),
    FundsDeposited(String, u32),
}

enum BankCommand {
    WithdrawFunds(String, u32),
    DepositFunds(String, u32),
}

#[derive(Debug, Clone)]
struct AccountData {
    acctnum: String,
    balance: u32,
    generation: u64,
}

impl AggregateState for AccountData {
    fn generation(&self) -> u64 {
        self.generation
    }
}

struct Account;

impl Aggregate for Account {
    type Event = BankEvent;
    type State = AccountData;
    type Command = BankCommand;

    fn apply_event(state: &Self::State, evt: &Self::Event) -> Result<Self::State> {
        let state = match *evt {
            BankEvent::FundsWithdrawn(_, amt) => AccountData {
                balance: state.balance - amt,
                acctnum: state.acctnum.to_owned(),
                generation: state.generation + 1,
            },
            BankEvent::FundsDeposited(_, amt) => AccountData {
                balance: state.balance + amt,
                acctnum: state.acctnum.to_owned(),
                generation: state.generation + 1,
            },
        };
        Ok(state)
    }

    fn handle_command(_state: &Self::State, cmd: &Self::Command) -> Result<Vec<Self::Event>> {
        // SHOULD DO: validate state and command

        // if validation passes...
        let evts = match cmd {
            BankCommand::DepositFunds(acct, amt) => {
                vec![BankEvent::FundsDeposited(acct.clone(), *amt)]
            }
            BankCommand::WithdrawFunds(acct, amt) => {
                vec![BankEvent::FundsWithdrawn(acct.clone(), *amt)]
            }
        };
        Ok(evts)
    }
}

fn main() {
    let _account_store = MemoryEventStore::new();

    let deposit = BankCommand::DepositFunds("SAVINGS100".to_string(), 500);

    let initial_state = AccountData {
        balance: 800,
        acctnum: "SAVINGS100".to_string(),
        generation: 1,
    };

    let post_deposit = Account::handle_command(&initial_state, &deposit).unwrap();
    let state = Account::apply_event(&initial_state, &post_deposit[0]).unwrap();

    println!("{:#?}", post_deposit);
    println!("{:#?}", state);
}
