extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Blueprint)]
pub fn blueprint_derive(input: TokenStream) -> TokenStream {
    let stringified_input = &input.to_string();
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let stringified_name = name.to_string().to_lowercase();

    let gen = quote! {
        impl Blueprint for #name {
            fn get_blueprint() -> String {
                #stringified_input.to_string()
            }

            fn get_table() -> String {
                #stringified_name.to_string()
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(Transferable)]
pub fn transferable_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    println!("{:?}", ast);
    let name = &ast.ident;

    let gen = quote! {
        impl Transferable for #name {
            fn serialize(&self) -> String {
                let result = Vec::new();

                String::from_utf8(result).unwrap()
            }
        }
    };

    gen.into()
}