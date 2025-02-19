#![doc = include_str!("../README.md")]

use shared::{Attribute, Struct};
use syn::{Ident, Type};

/// Returns the implementation of the `Struct` and `Foo` traits for the given struct.
pub fn implements_struct_and_foo(s: &Struct) -> proc_macro2::TokenStream {
    let struct_ident = Ident::new(&s.name, proc_macro2::Span::call_site());

    let attributes = s.attributes.iter().map(|a: &Attribute| {
        let attribute_ident = Ident::new(&a.name, proc_macro2::Span::call_site());
        let r#type: Type = syn::parse_str(&a.r#type).unwrap();
        let optional = a.optional;

        if optional {
            quote::quote! {
                pub #attribute_ident: Option<#r#type>,
            }
        } else {
            quote::quote! {
                pub #attribute_ident: #r#type,
            }
        }
    });

    let number_of_attributes = s.number_of_attributes();
    let number_of_optional_fields = s.number_of_optional_fields();

    quote::quote! {
        pub struct #struct_ident {
            #(#attributes)*
        }

        impl shared::Foo for #struct_ident {
            fn number_of_attributes() -> usize {
                #number_of_attributes
            }

            fn number_of_optional_fields() -> usize {
                #number_of_optional_fields
            }
        }
    }
}
