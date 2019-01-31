extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed, Ident,
    IntSuffix, LitInt, WhereClause,
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

    let ty;
    let ty_predicates;
    let into;
    let from;
    let imp = match data {
        Data::Struct(DataStruct { fields, .. }) => {
            ty = fields
                .iter()
                .fold(quote! { ::generics::Unit }, |acc, field| {
                    let field_ty = &field.ty;
                    quote! { ::generics::Prod<#acc, <#field_ty as ::generics::Generic>::Repr> }
                });
            ty_predicates = fields
                .iter()
                .map(|field| {
                    let field_ty = &field.ty;
                    quote! { #field_ty : ::generics::Generic }
                })
                .collect::<Vec<_>>();
            let ref self_fields = fields
                .iter()
                .enumerate()
                .map(|(i, field)| match &field.ident {
                    Some(ident) => quote! { #ident },
                    None => {
                        let lit = LitInt::new(i as u64, IntSuffix::None, Span::call_site());
                        quote! { #lit }
                    }
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
            let into_conversions = ordinals.iter().map(|ordinal| {
                quote! { let #ordinal = ::generics::Generic::into_repr(#ordinal); }
            });
            let from_conversions = ordinals.iter().map(|ordinal| {
                quote! { let #ordinal = ::generics::Generic::from_repr(#ordinal); }
            });
            into = quote! {
                let Self { #(#self_fields : #ordinals),* } = self;
                #( #into_conversions )*
                #repr_structure
            };
            from = quote! {
                let #repr_structure = repr;
                #( #from_conversions )*
                Self { #(#self_fields : #ordinals),* }
            };
        }
        Data::Enum(DataEnum { variants, .. }) => {
            unimplemented!();
        }
        Data::Union(_) => panic!("`Generic` cannot be derived for unions"),
    };

    let combined_where_clause = match where_clause {
        Some(WhereClause {
            where_token: _,
            predicates,
        }) => {
            quote! {
                where #(#ty_predicates ,)* #predicates
            }
        }
        None => {
            quote! {
                where #(#ty_predicates ,)*
            }
        }
    };

    TokenStream::from(quote! {
        impl #impl_generics Generic for #name #ty_generics #combined_where_clause {
            type Repr = #ty;
            fn into_repr(self: Self) -> Self::Repr {
                #into
            }
            fn from_repr(repr: Self::Repr) -> Self {
                #from
            }
        }
    })
}
