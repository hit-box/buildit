use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::Ident;

use crate::{
    parser::{Field, Source},
    traits::StateTrait,
};

#[derive(Debug)]
pub(crate) struct FieldState<'a> {
    pub(crate) field: &'a Field<'a>,
    state_trait: &'a StateTrait<'a>,
    pub(crate) ident: Ident,
}

impl<'a> FieldState<'a> {
    pub(crate) fn new(
        source: &'a Source,
        field: &'a Field<'a>,
        state_trait: &'a StateTrait,
    ) -> Self {
        let ident = format_ident!(
            "{}Builder{}Set",
            source.ident,
            field
                .attributes
                .ident
                .as_ref()
                .unwrap()
                .to_string()
                .to_case(Case::Pascal)
        );
        Self {
            field,
            ident,
            state_trait,
        }
    }
}

impl<'a> ToTokens for FieldState<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            ident,
            state_trait,
            field,
            ..
        } = self;
        let state_trait_ident = &state_trait.ident;
        let field_ident = &field.attributes.ident;
        let field_type = &field.attributes.ty;
        let lifetimes = field.lifetimes.iter().collect::<Vec<_>>();
        let generics = field.generics.iter().collect::<Vec<_>>();
        tokens.extend(quote! {
            #[derive(Debug)]
            pub struct #ident<#( #lifetimes, )* #( #generics, )* InnerBuilderState: #state_trait_ident> {
                inner: InnerBuilderState,
                #field_ident: Option<#field_type>,
            }
            impl<#( #lifetimes, )* #( #generics, )* InnerBuilderState: #state_trait_ident> #state_trait_ident for #ident<#( #lifetimes, )* #( #generics, )* InnerBuilderState> {}
        })
    }
}

#[derive(Debug)]
pub(crate) struct InitialState {
    pub(crate) ident: Ident,
}

impl InitialState {
    pub(crate) fn new(source: &Source) -> Self {
        let ident = format_ident!("{}Builder", &source.ident);
        InitialState { ident }
    }
}

impl ToTokens for InitialState {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ident, .. } = self;
        tokens.extend(quote! {
            #[derive(Debug)]
            pub struct #ident;
        })
    }
}
