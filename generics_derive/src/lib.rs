extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data};

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

    match data {
        Data::Enum(variants) => {
            unimplemented!()
        }
        Data::Struct(variant) => {
            unimplemented!()
        }
        Data::Union(_) => panic!("`Generic` cannot be derived for unions"),
    }

    let gen = quote! {
        impl #impl_generics Generic for #name #ty_generics #where_clause {
            type Repr = ();
            fn into_repr(self: Self) -> Self::Repr {
                unimplemented!();
            }
            fn from_repr(repr: Self::Repr) -> Self {
                unimplemented!();
            }
        }
    };

    TokenStream::from(gen)
}
