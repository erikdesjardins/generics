extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed, Ident,
};

#[proc_macro_derive(Generic)]
pub fn generic_macro_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: name,
        vis: _,
        attrs: _,
        generics,
        data,
    } = syn::parse(input).unwrap();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let imp = match data {
        Data::Struct(DataStruct { fields, .. }) => {
            let ty = fields
                .iter()
                .fold(quote! { ::generics::Unit }, |acc, field| {
                    let field_ty = &field.ty;
                    quote! { ::generics::Prod<#acc, #field_ty> }
                });
            let ref self_fields = fields
                .iter()
                .enumerate()
                .map(|(i, field)| {
                    field
                        .ident
                        .clone()
                        .unwrap_or_else(|| Ident::new(&i.to_string(), Span::call_site()))
                })
                .collect::<Vec<_>>();
            let ref ordinals = fields
                .iter()
                .enumerate()
                .map(|(i, _)| Ident::new(&format!("_{}", i), Span::call_site()))
                .collect::<Vec<_>>();
            let repr_structure =
                ordinals
                    .iter()
                    .fold(quote! { ::generics::Unit }, |acc, ordinal| {
                        quote! { ::generics::Prod(#acc, #ordinal) }
                    });
            quote! {
                type Repr = #ty;
                fn into_repr(self: Self) -> Self::Repr {
                    let Self { #(#self_fields : #ordinals),* } = self;
                    #repr_structure
                }
                fn from_repr(repr: Self::Repr) -> Self {
                    let #repr_structure = repr;
                    Self { #(#self_fields : #ordinals ,)* }
                }
            }
        }
        Data::Enum(DataEnum { variants, .. }) => {
            unimplemented!();
        }
        Data::Union(_) => panic!("`Generic` cannot be derived for unions"),
    };

    TokenStream::from(quote! {
        impl #impl_generics Generic for #name #ty_generics #where_clause {
            #imp
        }
    })
}
