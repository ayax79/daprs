extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Stateful)]
pub fn stateful_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_stateful_macro(&ast)
}

fn impl_stateful_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Stateful for #name {
            fn key() -> &'static str {
                stringify!(#name)
            }
        }
    };
    gen.into()
}