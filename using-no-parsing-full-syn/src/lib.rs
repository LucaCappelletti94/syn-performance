#![doc = include_str!("../README.md")]

use proc_macro2::{Span, TokenStream};
use shared::Struct;
use syn::__private::ToTokens;
use syn::{
    punctuated::Punctuated, token::Comma, token::PathSep, AngleBracketedGenericArguments, Block,
    Expr, ExprLit, Field, FieldMutability, FieldsNamed, GenericArgument, Ident, ImplItem,
    ImplItemFn, ItemImpl, ItemStruct, Lit, LitInt, Path, PathArguments, PathSegment, ReturnType,
    Type, TypePath,
};

/// Returns the implementation of the `Struct` and `Foo` traits for the given struct.
pub fn implements_struct_and_foo(s: &Struct) -> TokenStream {
    let struct_ident = Ident::new(&s.name, Span::call_site());

    // Build the fields for the struct.
    let mut fields = Punctuated::<Field, Comma>::new();
    for a in &s.attributes {
        let attribute_ident = Ident::new(&a.name, Span::call_site());
        let base_type: Type = syn::parse_str(&a.r#type).unwrap();

        let field_type = if a.optional {
            // Build a type of the form `Option<base_type>`
            Type::Path(TypePath {
                qself: None,
                path: {
                    let mut segments = Punctuated::<PathSegment, PathSep>::new();
                    segments.push(PathSegment {
                        ident: Ident::new("Option", Span::call_site()),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Default::default(),
                            args: {
                                let mut args = Punctuated::new();
                                args.push_value(GenericArgument::Type(base_type));
                                args
                            },
                            gt_token: Default::default(),
                        }),
                    });
                    Path {
                        leading_colon: None,
                        segments,
                    }
                },
            })
        } else {
            base_type
        };

        let field = Field {
            attrs: Vec::new(),
            vis: syn::Visibility::Public(Default::default()),
            mutability: FieldMutability::None,
            ident: Some(attribute_ident),
            colon_token: Some(Default::default()),
            ty: field_type,
        };
        fields.push(field);
    }
    let fields_named = FieldsNamed {
        brace_token: Default::default(),
        named: fields,
    };

    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: syn::Visibility::Public(Default::default()),
        struct_token: Default::default(),
        ident: struct_ident.clone(),
        generics: Default::default(),
        fields: syn::Fields::Named(fields_named),
        semi_token: None,
    };

    // Create functions for the Foo trait.
    let number_of_attributes = s.number_of_attributes();
    let number_of_optional_fields = s.number_of_optional_fields();

    let fn_number_of_attributes = ImplItem::Fn(ImplItemFn {
        attrs: Vec::new(),
        vis: syn::Visibility::Inherited,
        defaultness: None,
        sig: syn::Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: Ident::new("number_of_attributes", Span::call_site()),
            generics: Default::default(),
            paren_token: Default::default(),
            inputs: Punctuated::new(),
            variadic: None,
            output: ReturnType::Type(
                Default::default(),
                Box::new(Type::Path(TypePath {
                    qself: None,
                    path: {
                        let mut segments = Punctuated::<PathSegment, PathSep>::new();
                        segments.push_value(PathSegment {
                            ident: Ident::new("usize", Span::call_site()),
                            arguments: PathArguments::None,
                        });
                        Path {
                            leading_colon: None,
                            segments,
                        }
                    },
                })),
            ),
        },
        block: Block {
            brace_token: Default::default(),
            stmts: vec![syn::Stmt::Expr(
                Expr::Lit(ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::Int(LitInt::new(
                        &number_of_attributes.to_string(),
                        Span::call_site(),
                    )),
                }),
                None,
            )],
        },
    });

    let fn_number_of_optional_fields = ImplItem::Fn(ImplItemFn {
        attrs: Vec::new(),
        vis: syn::Visibility::Inherited,
        defaultness: None,
        sig: syn::Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: Ident::new("number_of_optional_fields", Span::call_site()),
            generics: Default::default(),
            paren_token: Default::default(),
            inputs: Punctuated::new(),
            variadic: None,
            output: ReturnType::Type(
                Default::default(),
                Box::new(Type::Path(TypePath {
                    qself: None,
                    path: {
                        let mut segments = Punctuated::<PathSegment, PathSep>::new();
                        segments.push_value(PathSegment {
                            ident: Ident::new("usize", Span::call_site()),
                            arguments: PathArguments::None,
                        });
                        Path {
                            leading_colon: None,
                            segments,
                        }
                    },
                })),
            ),
        },
        block: Block {
            brace_token: Default::default(),
            stmts: vec![syn::Stmt::Expr(
                Expr::Lit(ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::Int(LitInt::new(
                        &number_of_optional_fields.to_string(),
                        Span::call_site(),
                    )),
                }),
                None,
            )],
        },
    });

    // Build the impl block for `impl shared::Foo for <struct_ident>`
    let impl_item = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: Default::default(),
        trait_: Some((
            Default::default(),
            {
                let mut segments = Punctuated::<PathSegment, PathSep>::new();
                segments.push_value(PathSegment {
                    ident: Ident::new("Foo", Span::call_site()),
                    arguments: PathArguments::None,
                });
                Path {
                    leading_colon: None,
                    segments,
                }
            },
            Default::default(),
        )),
        self_ty: Box::new(Type::Path(TypePath {
            qself: None,
            path: {
                let mut segments = Punctuated::<PathSegment, PathSep>::new();
                segments.push_value(PathSegment {
                    ident: struct_ident.clone(),
                    arguments: PathArguments::None,
                });
                Path {
                    leading_colon: None,
                    segments,
                }
            },
        })),
        brace_token: Default::default(),
        items: vec![fn_number_of_attributes, fn_number_of_optional_fields],
    };

    // Combine the struct and impl definitions.
    let mut tokens = TokenStream::new();
    tokens.extend(item_struct.into_token_stream());
    tokens.extend(impl_item.into_token_stream());
    tokens
}
