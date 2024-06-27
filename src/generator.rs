use convert_case::{Case, Casing};
use darling::{
    ast::Data, usage::GenericsExt, util::Ignored, FromDeriveInput, FromField, FromMeta,
    FromMetaItem, FromTypeParam,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Error, Generics, Ident, Type, TypeReference};

#[derive(Debug)]
pub(crate) struct Generator<'a> {
    source: &'a Source,
    state_trait: &'a StateTrait<'a>,
    initial_state: &'a InitialState<'a>,
}

impl<'a> Generator<'a> {
    pub(crate) fn new(
        source: &'a Source,
        state_trait: &'a StateTrait<'a>,
        initial_state: &'a InitialState<'a>,
    ) -> Result<Self, Error> {
        Ok(Generator {
            source,
            state_trait,
            initial_state,
        })
    }
}

impl<'a> ToTokens for Generator<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            source,
            initial_state,
            state_trait,
        } = self;
        let struct_name = &source.ident;
        let generics = &source.generics;
        let generic_params = &source
            .generics
            .declared_type_params()
            .into_iter()
            .collect::<Vec<_>>();
        let lifetimes = &source
            .generics
            .declared_lifetimes()
            .into_iter()
            // .map(|lifetime| lifetime.ident)
            .collect::<Vec<_>>();
        let where_clause = &source.generics.where_clause;
        let initial_state_ident = &initial_state.ident;
        let module_name = format_ident!("{}_builder", struct_name.to_string().to_case(Case::Snake));
        let imports = Imports::new(initial_state, &module_name);
        let field_states = match &source.data {
            Data::Struct(fields) => fields
                .iter()
                .map(|field| FieldState::new(source, field, state_trait))
                .collect::<Vec<_>>(),
            _ => unimplemented!(),
        };
        tokens.extend(quote! {
            mod #module_name {
                use super::*;
                impl #generics #struct_name <#(#lifetimes,)* #(#generic_params,)*> #where_clause {
                    pub fn builder() -> #initial_state_ident { #initial_state_ident {} }
                }
                #state_trait
                #initial_state
                #( #field_states )*
            }
            #imports

        })
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(builder))]
pub(crate) struct Field {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    skip: bool,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(lorem), supports(struct_named))]
pub(crate) struct Source {
    ident: Ident,
    // generics: Generics,
    generics: Generics,
    data: Data<Ignored, Field>,
    // fields: Fields<Field>,
}

#[derive(Debug)]
pub(crate) struct InitialState<'a> {
    source: &'a Source,
    state_trait: &'a StateTrait<'a>,
    ident: Ident,
}

impl<'a> InitialState<'a> {
    pub(crate) fn new(source: &'a Source, state_trait: &'a StateTrait) -> Self {
        let ident = format_ident!("{}Builder", &source.ident);
        InitialState {
            source,
            ident,
            state_trait,
        }
    }
}

impl<'a> ToTokens for InitialState<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            ident, state_trait, ..
        } = self;
        let state_trait_ident = &state_trait.ident;
        tokens.extend(quote! {
            #[derive(Debug)]
            pub struct #ident;
            impl #state_trait_ident for #ident {}
        })
    }
}

pub(crate) struct Imports<'a> {
    initial_state: &'a InitialState<'a>,
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
        tokens.extend(quote! {
            pub use #module::#initial_state_ident;
        })
    }
}

#[derive(Debug)]
pub(crate) struct StateTrait<'a> {
    source: &'a Source,
    ident: Ident,
}

impl<'a> StateTrait<'a> {
    pub(crate) fn new(source: &'a Source) -> Self {
        let ident = format_ident!("{}BuilderState", source.ident);
        Self { source, ident }
    }
}

impl<'a> ToTokens for StateTrait<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { ident, .. } = self;
        tokens.extend(quote! {
            pub trait #ident {}
        })
    }
}

pub(crate) struct FieldState<'a> {
    source: &'a Source,
    field: &'a Field,
    state_trait: &'a StateTrait<'a>,
    ident: Ident,
}

impl<'a> FieldState<'a> {
    pub(crate) fn new(source: &'a Source, field: &'a Field, state_trait: &'a StateTrait) -> Self {
        let ident = format_ident!(
            "{}Builder{}Set",
            source.ident,
            field
                .ident
                .as_ref()
                .unwrap()
                .to_string()
                .to_case(Case::Pascal)
        );
        Self {
            source,
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
            source,
            ..
        } = self;
        let state_trait_ident = &state_trait.ident;
        let field_ident = &field.ident;
        let field_type = &field.ty;
        let (lifetime, type_ident) = match field_type {
            Type::Path(ty) => (vec![], ty.path.get_ident()),
            Type::Reference(TypeReference {
                elem: ty, lifetime, ..
            }) => match ty.as_ref() {
                Type::Path(ty) => (vec![lifetime], ty.path.get_ident()),
                _ => (vec![], None),
            },
            _ => (vec![], None),
        };
        let generic_type_param = source
            .generics
            .type_params()
            .find(|&t| Some(&t.ident) == type_ident);
        let generic_type_param_ident = generic_type_param.map(|type_param| &type_param.ident);
        dbg!(&generic_type_param, &generic_type_param_ident);
        tokens.extend(quote! {
            pub struct #ident<#(#lifetime,)* I: #state_trait_ident, #generic_type_param> {
                inner: I,
                #field_ident: Option<#field_type>,
            }
            impl<#(#lifetime,)* I: #state_trait_ident, #generic_type_param> #state_trait_ident for #ident<#(#lifetime,)* I, #generic_type_param_ident> {}
        })
    }
}
