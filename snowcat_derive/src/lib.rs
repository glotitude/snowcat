extern crate proc_macro;

use proc_macro2::{Ident, Span};
use proc_macro::TokenStream;
use quote::quote;
use syn::*;
use syn::PathSegment;

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
    let name = &ast.ident;

    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct{struct_token, fields, semi_token}) => fields,
        _ => panic!(""),
    };

    let named = match fields {
        syn::Fields::Named(syn::FieldsNamed{ brace_token, named }) => named,
        _ => panic!("")
    };

    let mut fields_idents = Vec::new();
    let mut fields_names = Vec::new();
    for n in named {
        let tmp = match n.ident {
            Some(e) => e.to_string(),
            None => String::new(),
        };

        let mut p = syn::punctuated::Punctuated::new();
        p.push(PathSegment::from(Ident::new("self", Span::call_site())));

        let ef = syn::ExprField {
            attrs: Vec::new(),
            base: Box::new(syn::Expr::Path(syn::ExprPath {
                attrs: vec![],
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: p,
                }
            })),
            dot_token: syn::Token![.](Span::call_site()),
            member: syn::Member::Named(Ident::new(&tmp, Span::call_site()))
        };

        fields_names.push(tmp);
        fields_idents.push(ef);
    }

    let gen = quote! {
        impl Transferable for #name {
            fn serialize(&self) -> String {
                let mut result = String::new();

                #(
                    result.push_str(&format!("{}={}\n", #fields_names, #fields_idents.to_string()));
                )*

                result
            }
        }
    };

    gen.into()
}