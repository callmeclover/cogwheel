use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Ident, Type};

#[proc_macro_derive(Configuration)]
#[allow(clippy::missing_panics_doc)]
pub fn configuration_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);
    impl_configuration(&ast)
}

fn impl_configuration(ast: &syn::DeriveInput) -> TokenStream {
    let name: &Ident = &ast.ident;
    let gen: proc_macro2::TokenStream = quote! {
        impl Configuration for #name {}
    };
    gen.into()
}

/// A macro which generates a struct from `T`, let's call it `TSparse`,
/// where all keys of `T` (recursive) are `Option`.
/// 
/// Anything using `replace*` on a builder **must** have `#[with_sparse]`.
#[proc_macro_attribute]
pub fn with_sparse(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    let struct_name: &Ident = &input.ident;

    // Generate the new struct name by appending "Sparse"
    let sparse_struct_name: Ident = syn::Ident::new(&format!("{}Sparse", struct_name), struct_name.span());

    // Collect field information and wrap in `Option`
    let fields: Vec<proc_macro2::TokenStream> = if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            fields_named.named.iter().map(|f: &Field| {
                let field_name: &Option<Ident> = &f.ident;
                let field_ty: &Type = &f.ty;
                quote! {
                    #field_name: Option<#field_ty>
                }
            }).collect::<Vec<_>>()
        } else {
            panic!("#[sparse] can only be applied to structs with named fields.");
        }
    } else {
        panic!("#[sparse] can only be applied to structs.");
    };

    // Include traits that you want the sparse struct to derive
    let derives: proc_macro2::TokenStream = quote! {
        #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, cogwheel::Configuration)]
    };

    // Generate the sparse struct definition
    let expanded: proc_macro2::TokenStream = quote! {
        // Keep the original struct as-is
        #input

        // Define the sparse struct with derived traits
        #derives
        pub struct #sparse_struct_name {
            #(#fields),*
        }

        impl cogwheel::Sparse for #sparse_struct_name {}
    };

    TokenStream::from(expanded)
}
