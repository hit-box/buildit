use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use states::{FieldState, InitialState};
use syn::{parse_macro_input, DeriveInput, Error};
use traits::{
    FieldGetterTrait, FieldGetterTraitDefaultImpl, FieldGetterTraitImpl, FieldSetterTrait,
};

use crate::generator::Generator;
use crate::parser::Source;
use crate::traits::StateTrait;

mod generator;
mod parser;
mod states;
mod traits;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_buildit(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // test for types test
    let input = parse_macro_input!(input as DeriveInput);
    match derive_imp(&input) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn derive_imp(input: &DeriveInput) -> Result<TokenStream, Error> {
    let source = Source::from_derive_input(input)?;
    let fields = source.struct_fields()?.collect::<Vec<_>>();
    let field_getter_traits = fields
        .iter()
        .map(|field| FieldGetterTrait::new(&source, field))
        .collect::<Vec<_>>();
    let state_trait = StateTrait::new(&source, field_getter_traits.iter().collect());
    let initial_state = InitialState::new(&source);
    let field_states = fields
        .iter()
        .map(|field| FieldState::new(&source, field, &state_trait))
        .collect::<Vec<_>>();
    let fields_getter_trait_impls = field_states
        .iter()
        .flat_map(|state| {
            field_getter_traits
                .iter()
                .map(|getter| FieldGetterTraitImpl::new(getter, state))
        })
        .collect::<Vec<_>>();
    let field_setter_traits = fields
        .iter()
        .zip(field_states.iter())
        .map(|(field, field_state)| {
            FieldSetterTrait::new(&source, field, &state_trait, field_state)
        })
        .collect::<Vec<_>>();
    let default_getters_impls =
        field_getter_traits
            .iter()
            .filter_map(|getter| {
                getter.field.attributes.default.as_ref().map(|default| {
                    FieldGetterTraitDefaultImpl::new(getter, &initial_state, default)
                })
            })
            .collect::<Vec<_>>();
    let generator = Generator::new(
        &source,
        &state_trait,
        &initial_state,
        &field_states,
        &field_setter_traits,
        &field_getter_traits,
        &fields_getter_trait_impls,
        &default_getters_impls,
    )?;
    Ok(quote! { #generator })
}
