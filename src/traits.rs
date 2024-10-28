use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::Ident;

use crate::{
    parser::{Field, Source},
    states::{FieldState, InitialState},
};

#[derive(Debug)]
pub(crate) struct StateTrait<'a> {
    source: &'a Source,
    field_getter_traits: Vec<&'a FieldGetterTrait<'a>>,
    pub(crate) ident: Ident,
}

impl<'a> StateTrait<'a> {
    pub(crate) fn new(
        source: &'a Source,
        field_getter_traits: Vec<&'a FieldGetterTrait<'a>>,
    ) -> Self {
        let ident = format_ident!("{}State", source.base_ident());
        Self {
            source,
            ident,
            field_getter_traits,
        }
    }

    pub(crate) fn impl_for(&self, state: &'a InitialState) -> StateTraitImpl {
        StateTraitImpl::new(self, state)
    }
}

impl<'a> ToTokens for StateTrait<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            ident,
            field_getter_traits,
            source,
            ..
        } = self;
        let source_ident = &source.ident;
        let getter_traits = field_getter_traits
            .iter()
            .map(|getter_trait| getter_trait.ty());
        let source_fields = field_getter_traits.iter().map(|getter| {
            let field = &getter.field.attributes.ident;
            let getter_fn = &getter.fn_ident;
            quote! {
                #field: self.#getter_fn()
            }
        });
        let (impl_generics, ty_generics, where_clause) = source.generics.split_for_impl();
        let where_clause = where_clause
            .map(|clause| quote! { #clause })
            .unwrap_or_else(|| quote! { where });
        tokens.extend(quote! {
            pub trait #ident {
                fn build #impl_generics (mut self) -> #source_ident #ty_generics
                #where_clause
                Self: #( #getter_traits+ )* Sized,
                {
                    #source_ident {
                        #( #source_fields, )*
                    }
                }
            }
        })
    }
}

pub struct StateTraitImpl<'a> {
    trt: &'a StateTrait<'a>,
    implementor: &'a InitialState,
}

impl<'a> StateTraitImpl<'a> {
    pub(crate) fn new(trt: &'a StateTrait, implementor: &'a InitialState) -> Self {
        Self { trt, implementor }
    }
}

impl<'a> ToTokens for StateTraitImpl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { trt, implementor } = self;
        let trait_ident = &trt.ident;
        let implementor_ident = &implementor.ident;
        tokens.extend(quote! {
            impl #trait_ident for #implementor_ident {}
        })
    }
}

#[derive(Debug)]
pub(crate) struct FieldSetterTrait<'a> {
    ident: Ident,
    state_trait: &'a StateTrait<'a>,
    field_state: &'a FieldState<'a>,
    field: &'a Field<'a>,
}

impl<'a> FieldSetterTrait<'a> {
    pub(crate) fn new(
        source: &'a Source,
        field: &'a Field<'a>,
        state_trait: &'a StateTrait,
        field_state: &'a FieldState,
    ) -> Self {
        let ident = format_ident!(
            "{}BuilderSet{}",
            &source.ident,
            field
                .attributes
                .ident
                .as_ref()
                .unwrap()
                .to_string()
                .to_case(Case::Pascal)
        );
        Self {
            ident,
            state_trait,
            field_state,
            field,
        }
    }
}

impl<'a> ToTokens for FieldSetterTrait<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            ident,
            state_trait,
            field_state,
            field,
            ..
        } = self;
        let field_ident = format_ident!("{}", field.attributes.ident.as_ref().unwrap().to_string());
        let field_state_ident = &field_state.ident;
        let state_trait_ident = &state_trait.ident;
        let field_type = &field.attributes.ty;
        let lifetimes = field.lifetimes.iter().collect::<Vec<_>>();
        let generics = field.generics.iter().collect::<Vec<_>>();
        let generics_declaration = field.generics_declaration();
        tokens.extend(quote! {
            pub trait #ident: Sized + #state_trait_ident {
                fn #field_ident #generics_declaration (self, value: #field_type) -> #field_state_ident< #( #lifetimes, )* #( #generics, )* Self> {
                    #field_state_ident {
                        inner: self,
                        #field_ident: Some(value),
                    }
                }
            }
            impl<T: #state_trait_ident> #ident for T {}
        })
    }
}

#[derive(Debug)]
pub(crate) struct FieldGetterTrait<'a> {
    ident: Ident,
    fn_ident: Ident,
    field: &'a Field<'a>,
}

// @TODO: add field's generics and lifetimes
impl<'a> FieldGetterTrait<'a> {
    pub(crate) fn new(source: &'a Source, field: &'a Field) -> Self {
        let base_ident = &source.base_ident();
        let ident = format_ident!(
            "{base_ident}Get{}",
            field
                .attributes
                .ident
                .as_ref()
                .unwrap()
                .to_string()
                .to_case(Case::Pascal)
        );
        let fn_ident = format_ident!("get_{}", field.attributes.ident.as_ref().unwrap());
        Self {
            ident,
            field,
            fn_ident,
        }
    }

    pub(crate) fn generics_declaration(&self) -> Option<TokenStream> {
        self.field.generics_declaration()
    }

    fn ty(&self) -> TokenStream {
        let Self { ident, field, .. } = self;
        let generics = if field.lifetimes.is_empty() && field.generics.is_empty() {
            None
        } else {
            let generics = field.generics.iter();
            let lifetimes = field.lifetimes.iter();
            Some(quote! {
                <#(#lifetimes,)* #(#generics,)*>
            })
        };
        quote! { #ident #generics }
    }
}

impl<'a> ToTokens for FieldGetterTrait<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            ident,
            field,
            fn_ident,
            ..
        } = self;
        let return_ty = &field.attributes.ty;
        let generics_declaration = field.generics_declaration();

        tokens.extend(quote! {
            #[diagnostic::on_unimplemented(
                message = "HOST My Message for `SourceBuilder` is not implemented for `{Self}`",
                label = "My Label",
                note = "Note 1",
                note = "Note 2"
            )]
            pub trait #ident #generics_declaration {
                fn #fn_ident(&mut self) -> #return_ty;
            }
        })
    }
}

#[derive(Debug)]
pub(crate) struct FieldGetterTraitImpl<'a> {
    getter_trait: &'a FieldGetterTrait<'a>,
    state: &'a FieldState<'a>,
    state_trait_ident: Ident,
}

impl<'a> FieldGetterTraitImpl<'a> {
    pub(crate) fn new(getter_trait: &'a FieldGetterTrait<'a>, state: &'a FieldState<'a>) -> Self {
        let state_trait_ident = format_ident!("{}State", getter_trait.field.source.base_ident());
        Self {
            state_trait_ident,
            getter_trait,
            state,
        }
    }
}

impl<'a> ToTokens for FieldGetterTraitImpl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            state_trait_ident,
            getter_trait,
            state,
        } = self;
        let getter_trait_ident = &getter_trait.ident;
        let state_ident = &state.ident;
        let getter_fn_ident = &getter_trait.fn_ident;
        let getter_generics_declaration = getter_trait.generics_declaration();
        let field_ty = &getter_trait.field.attributes.ty;
        let field_ident = &state.field.attributes.ident;
        let state_lifetimes = &state.field.lifetimes.iter().collect::<Vec<_>>();
        let state_generics = &state.field.generics.iter().collect::<Vec<_>>();
        let union_generics = state
            .field
            .generics
            .union(&getter_trait.field.generics)
            .collect::<Vec<_>>();
        let union_lifetimes = state
            .field
            .lifetimes
            .union(&getter_trait.field.lifetimes)
            .collect::<Vec<_>>();

        if getter_trait.field.attributes.ident == state.field.attributes.ident {
            tokens.extend(quote! {
                impl<#( #union_lifetimes, )* #( #union_generics, )* InnerBuilderState: #state_trait_ident> #getter_trait_ident #getter_generics_declaration for #state_ident<#( #state_lifetimes, )* #( #state_generics, )* InnerBuilderState> {
                    fn #getter_fn_ident(&mut self) -> #field_ty {
                        self.#field_ident.take().expect("@TODO")
                    }
                }
            });
        } else {
            tokens.extend(quote! {
                impl<#( #union_lifetimes, )* #( #union_generics, )* InnerBuilderState: #state_trait_ident + #getter_trait_ident #getter_generics_declaration> #getter_trait_ident #getter_generics_declaration for #state_ident<#( #state_lifetimes, )* #( #state_generics, )* InnerBuilderState> {
                    fn #getter_fn_ident(&mut self) -> #field_ty {
                        self.inner.#getter_fn_ident()
                    }
                }
            });
        }
    }
}
