use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn bioscaleupgrade(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_derive(BioscaleUpgrade)]
pub fn derive_bioscale_upgrade(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl #name {
            pub fn evidence_tags(&self) -> Vec<String> {
                vec![]
            }
        }
    };
    expanded.into()
}
