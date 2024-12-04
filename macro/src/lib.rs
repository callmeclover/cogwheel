use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident};

#[proc_macro_derive(Configuration)]
#[allow(clippy::missing_panics_doc)]
pub fn configuration_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_configuration(&ast)
}

fn impl_configuration(ast: &syn::DeriveInput) -> TokenStream {
    let name: &Ident = &ast.ident;
    let gen: proc_macro2::TokenStream = quote! {
        impl Configuration for #name {}
    };
    gen.into()
}
