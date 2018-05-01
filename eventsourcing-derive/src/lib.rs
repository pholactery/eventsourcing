#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::synom::Synom;
use syn::{DeriveInput, Path};

/*impl Dispatcher for CombatDispatcher {
    type Command = CombatCommand;
    type Event = CombatEvent;
    type State = CombatState;
    type Aggregate = Combat;

    fn dispatch(state: &Self::State, cmd: Self::Command) -> Result<()> {
        match Self::Aggregate::handle_command(state, cmd) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}*/

#[proc_macro_derive(Dispatcher, attributes(event, command, state, aggregate))]
pub fn component(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let gen = impl_component(&ast);
    gen.into()
}

struct CommandAttribute {
    command: Path,
}

impl Synom for CommandAttribute {
    named!(parse -> Self, map!(
        parens!(syn!(Path)),
        |(_, command)| CommandAttribute { command }
    ));
}

struct StateAttribute {
    state: Path,
}

impl Synom for StateAttribute {
    named!(parse -> Self, map!(
        parens!(syn!(Path)),
        |(_, state)| StateAttribute { state }
    ));
}

struct AggregateAttribute {
    aggregate: Path,
}

impl Synom for AggregateAttribute {
    named!(parse -> Self, map!(
        parens!(syn!(Path)),
        |(_, aggregate)| AggregateAttribute { aggregate }
    ));
}

struct EventAttribute {
    event: Path,
}

impl Synom for EventAttribute {
    named!(parse -> Self, map!(
        parens!(syn!(Path)),
        |(_, event)| EventAttribute { event }
    ));
}

fn impl_component(ast: &DeriveInput) -> Tokens {
    let name = &ast.ident;
    let (impl_generics, _ty_generics, where_clause) = ast.generics.split_for_impl();

    let event = ast.attrs
        .iter()
        .find(|attr| attr.path.segments[0].ident == "event")
        .map(|attr| {
            syn::parse2::<EventAttribute>(attr.tts.clone())
                .unwrap()
                .event
        })
        .unwrap_or_else(|| parse_quote!(NoEvent));

    let command = ast.attrs
        .iter()
        .find(|attr| attr.path.segments[0].ident == "command")
        .map(|attr| {
            syn::parse2::<CommandAttribute>(attr.tts.clone())
                .unwrap()
                .command
        })
        .unwrap_or_else(|| parse_quote!(NoCommand));

    let aggregate = ast.attrs
        .iter()
        .find(|attr| attr.path.segments[0].ident == "aggregate")
        .map(|attr| {
            syn::parse2::<AggregateAttribute>(attr.tts.clone())
                .unwrap()
                .aggregate
        })
        .unwrap_or_else(|| parse_quote!(NoAggregate));

    let state = ast.attrs
        .iter()
        .find(|attr| attr.path.segments[0].ident == "state")
        .map(|attr| {
            syn::parse2::<StateAttribute>(attr.tts.clone())
                .unwrap()
                .state
        })
        .unwrap_or_else(|| parse_quote!(NoCommand));

    quote! {
        impl #impl_generics ::eventsourcing::Dispatcher for #name #where_clause {
            type Event = #event;
            type Command = #command;
            type Aggregate = #aggregate;
            type State = #state;

             fn dispatch(state: &Self::State, cmd: Self::Command) -> Result<()> {
               match Self::Aggregate::handle_command(state, cmd) {
                  Ok(_) => Ok(()),
                  Err(e) => Err(e),
                }
             }
        }
    }
}
