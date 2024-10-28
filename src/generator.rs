use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Error, Ident};

use crate::{
    parser::Source,
    states::{FieldState, InitialState},
    traits::{FieldGetterTrait, FieldGetterTraitImpl, FieldSetterTrait, StateTrait},
};

#[derive(Debug)]
pub(crate) struct Generator<'a> {
    pub(crate) source: &'a Source,
    pub(crate) state_trait: &'a StateTrait<'a>,
    pub(crate) initial_state: &'a InitialState,
    pub(crate) field_states: &'a Vec<FieldState<'a>>,
    pub(crate) field_setter_traits: &'a Vec<FieldSetterTrait<'a>>,
    pub(crate) field_getter_traits: &'a Vec<FieldGetterTrait<'a>>,
    pub(crate) field_getter_trait_impls: &'a Vec<FieldGetterTraitImpl<'a>>,
}

impl<'a> Generator<'a> {
    pub(crate) fn new(
        source: &'a Source,
        state_trait: &'a StateTrait<'a>,
        initial_state: &'a InitialState,
        field_states: &'a Vec<FieldState>,
        field_setter_traits: &'a Vec<FieldSetterTrait<'a>>,
        field_getter_traits: &'a Vec<FieldGetterTrait<'a>>,
        field_getter_trait_impls: &'a Vec<FieldGetterTraitImpl<'a>>,
    ) -> Result<Self, Error> {
        Ok(Generator {
            source,
            state_trait,
            initial_state,
            field_states,
            field_setter_traits,
            field_getter_traits,
            field_getter_trait_impls,
        })
    }
}

impl<'a> ToTokens for Generator<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            source,
            initial_state,
            state_trait,
            field_states,
            field_setter_traits,
            field_getter_traits,
            field_getter_trait_impls,
            ..
        } = self;
        let struct_name = &source.ident;
        let (impl_generics, ty_generics, where_clause) = source.generics.split_for_impl();
        let impl_state_trait_for_initial_state = state_trait.impl_for(initial_state);
        let initial_state_ident = &initial_state.ident;
        let module_name = format_ident!("{}_builder", struct_name.to_string().to_case(Case::Snake));
        let imports = Imports::new(initial_state, &module_name);
        tokens.extend(quote! {
            mod #module_name {
                use super::*;
                impl #impl_generics #struct_name #ty_generics #where_clause {
                    pub fn builder() -> #initial_state_ident { #initial_state_ident {} }
                }
                #state_trait
                #initial_state
                #impl_state_trait_for_initial_state
                #( #field_states )*
                #( #field_setter_traits )*
                #( #field_getter_traits )*
                #( #field_getter_trait_impls )*
            }
            #imports

        })
    }
}

pub(crate) struct Imports<'a> {
    initial_state: &'a InitialState,
    module: &'a Ident,
}

impl<'a> Imports<'a> {
    pub(crate) fn new(initial_state: &'a InitialState, module: &'a Ident) -> Self {
        Imports {
            initial_state,
            module,
        }
    }
}

impl<'a> ToTokens for Imports<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            initial_state,
            module,
            ..
        } = self;
        let initial_state_ident = &initial_state.ident;
        // @FIX: remove *
        tokens.extend(quote! {
            pub use #module::{#initial_state_ident, *};
        })
    }
}
