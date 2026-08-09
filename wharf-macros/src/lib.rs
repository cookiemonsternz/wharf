extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Object)]
pub fn derive_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl Object for #name {
            fn id(&self) -> Uuid {
                self.id
            }
        }
    };

    TokenStream::from(expanded)
}
