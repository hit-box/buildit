use darling::{
    ast::Data,
    usage::{GenericsExt, IdentSet, LifetimeSet, Purpose, UsesLifetimes, UsesTypeParams},
    uses_lifetimes, uses_type_params,
    util::Ignored,
    FromDeriveInput, FromField,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, Generics, Ident, Type};

#[derive(Debug, FromField)]
#[darling(attributes(builder))]
pub(crate) struct FieldAttributes {
    pub(crate) ident: Option<Ident>,
    pub(crate) ty: Type,
}

uses_lifetimes!(FieldAttributes, ty);
uses_type_params!(FieldAttributes, ty);

#[derive(Debug)]
pub(crate) struct Field<'a> {
    pub(crate) source: &'a Source,
    pub(crate) attributes: &'a FieldAttributes,
    pub(crate) lifetimes: LifetimeSet,
    pub(crate) generics: IdentSet,
}

impl<'a> Field<'a> {
    pub(crate) fn new(source: &'a Source, attributes: &'a FieldAttributes) -> Self {
        let lifetimes = attributes.uses_lifetimes_cloned(
            &Purpose::Declare.into(),
            &source.generics.declared_lifetimes(),
        );
        let generics = attributes.uses_type_params_cloned(
            &Purpose::Declare.into(),
            &source.generics.declared_type_params(),
        );
        Self {
            source,
            attributes,
            lifetimes,
            generics,
        }
    }

    pub(crate) fn generics_declaration(&self) -> Option<TokenStream> {
        let Self {
            lifetimes,
            generics,
            ..
        } = self;
        let lifetimes = lifetimes.iter().collect::<Vec<_>>();
        let generics = generics.iter().collect::<Vec<_>>();
        if lifetimes.is_empty() && generics.is_empty() {
            None
        } else {
            Some(quote! {
                <#(#lifetimes,)* #(#generics,)*>
            })
        }
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(lorem), supports(struct_named))]
pub(crate) struct Source {
    pub(crate) ident: Ident,
    pub(crate) generics: Generics,
    pub(crate) data: Data<Ignored, FieldAttributes>,
}

impl Source {
    pub(crate) fn struct_fields(&self) -> Result<impl Iterator<Item = Field>, Error> {
        match &self.data {
            Data::Struct(fields) => {
                Ok(fields.iter().map(|attributes| Field::new(self, attributes)))
            }
            _ => Err(Error::new(
                self.ident.span(),
                "buildit supports only structs",
            )),
        }
    }

    pub(crate) fn base_ident(&self) -> Ident {
        format_ident!("{}Builder", self.ident)
    }
}
