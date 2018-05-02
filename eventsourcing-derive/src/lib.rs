#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use quote::Tokens;
use syn::synom::Synom;
use syn::{DeriveInput, Path, LitInt, Variant, Ident, Data, Fields, DataEnum};
use syn::punctuated::{Punctuated};
use syn::token::Comma;

#[proc_macro_derive(Dispatcher, attributes(aggregate))]
pub fn component(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let gen = impl_component(&ast);
    gen.into()
}

#[proc_macro_derive(Event, attributes(schema_version))]
pub fn component_event(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let gen = match ast.data {
        Data::Enum(ref data_enum) => impl_component_event(&ast, data_enum),
        Data::Struct(_) =>
            quote! {
                panic!("#[derive(Event)] is only defined for enums, not structs")
            },
        Data::Union(_) =>
            quote! {
                panic!("#[derive(Event)] is only defined for enums, not unions")
            }
    };

    gen.into()
}

struct SchemaVersionAttribute {
    schema_version: LitInt,
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

impl Synom for SchemaVersionAttribute {
    named!(parse -> Self, map!(
        parens!(syn!(LitInt)),
        |(_, schema_version) | SchemaVersionAttribute { schema_version }
    ));
}

fn impl_component_event(ast: &DeriveInput, data_enum: &DataEnum) -> Tokens {
    let name = &ast.ident;
    let variants = &data_enum.variants;
    let (impl_generics, _ty_generics, where_clause) = ast.generics.split_for_impl();
    let schema_version = ast.attrs
        .iter()
        .find(|attr| attr.path.segments[0].ident == "schema_version")
        .map(|attr| {
            let x = syn::parse2::<SchemaVersionAttribute>(attr.tts.clone())
                .unwrap()
                .schema_version;
            x
        })
        .unwrap_or_else(|| parse_quote!(NoSchemaVersion));

    let event_matches = generate_event_matches(&name, &variants);

    quote! {
        impl #impl_generics ::eventsourcing::Event for #name #where_clause {
            fn schema_version(&self) -> u32 {
                #schema_version
            }

            fn event_type(&self) -> &str {
                match self {
                    #(#event_matches)*
                }
            }
        }
    }
}

fn generate_event_matches(name: &Ident, variants: &Punctuated<Variant, Comma>) -> Vec<quote::Tokens> {
    let mut result = Vec::new();
    for (_idx, variant) in variants.iter().enumerate() {
        let id = &variant.ident;
        let et_name = event_type_name(name, id);
        let new = match variant.fields {
            Fields::Unit => quote! {
                #name::#id => #et_name,
            },
            Fields::Unnamed(ref fields) => {
                let idents: Vec<_> = fields.unnamed.pairs().map(|p| p.value().ident).collect();
                quote! {
                    #name::#id( #(_#idents,)* ) => #et_name,
                }
            },
            Fields::Named(ref fields) => {
                let idents: Vec<_> = fields.named.pairs().map(|p| p.value().ident).collect();
                quote! {
                    #name::#id { #(#idents,)* } => #et_name,
                }
            }
        };
        result.push(new);
    }
    result
}

fn event_type_name(name: &Ident, variant_id: &Ident) -> String {
    let name_s = name.to_string().to_lowercase();
    let variant_s = variant_id.to_string().to_lowercase();
    format!("{}.{}", name_s, variant_s)
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

            fn dispatch(
                state: &Self::State,
                cmd: Self::Command,
                store: &impl ::eventsourcing::eventstore::EventStore,
            ) -> Vec<Result<::eventsourcing::eventstore::EnrichedEvent>> {
                match Self::Aggregate::handle_command(state, cmd) {
                    Ok(evts) => evts.into_iter().map(|evt| store.append(evt)).collect(),
                    Err(e) => vec![Err(e)],
                }
            }
        }
    }
}
