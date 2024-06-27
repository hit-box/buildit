use darling::FromDeriveInput;
use generator::{InitialState, StateTrait};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error};

use crate::generator::{Generator, Source};

mod generator;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_buildit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _a = "test";
    // test for types test
    let input = parse_macro_input!(input as DeriveInput);
    match derive_imp(&input) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn derive_imp(input: &DeriveInput) -> Result<TokenStream, Error> {
    let source = Source::from_derive_input(input)?;
    let state_trait = StateTrait::new(&source);
    let initial_state = InitialState::new(&source, &state_trait);
    let generator = Generator::new(&source, &state_trait, &initial_state)?;
    dbg!(&generator);
    Ok(quote! { #generator })
}
