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

#[proc_macro_derive(Dispatcher, attributes(aggregate))]
pub fn component(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let gen = impl_component(&ast);
    gen.into()
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

fn impl_component(ast: &DeriveInput) -> Tokens {
    let name = &ast.ident;
    let (impl_generics, _ty_generics, where_clause) = ast.generics.split_for_impl();

    let aggregate = ast.attrs
        .iter()
        .find(|attr| attr.path.segments[0].ident == "aggregate")
        .map(|attr| {
            syn::parse2::<AggregateAttribute>(attr.tts.clone())
                .unwrap()
                .aggregate
        })
        .unwrap_or_else(|| parse_quote!(NoAggregate));

    quote! {
        impl #impl_generics ::eventsourcing::Dispatcher for #name #where_clause {
            type Aggregate = #aggregate;
            type Event = <#aggregate as Aggregate>::Event;
            type Command = <#aggregate as Aggregate>::Command;
            type State = <#aggregate as Aggregate>::State;

            fn dispatch(state: &Self::State, cmd: Self::Command) -> Result<()> {
                match Self::Aggregate::handle_command(state, cmd) {
                  Ok(_) => Ok(()),
                  Err(e) => Err(e),
                }
            }
        }
    }
}
