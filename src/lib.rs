//! # error-rules
//!
//! [![Latest Version](https://img.shields.io/crates/v/error-rules.svg)](https://crates.io/crates/error-rules)
//! [![docs](https://docs.rs/error-rules/badge.svg)](https://docs.rs/error-rules)
//!
//! ## Intro
//!
//! error-rules is a derive macro to implement error handler.
//! Error handler based on the enum.
//! Macro automatically implements conversion of any error type into the inner enum field.
//!
//! ## Error conversion
//!
//! `#[error_from]` attribute implements an automatically conversion from any error type.
//! Converted type should implements `std::error::Error` itnerface.
//!
//! ```rust
//! use error_rules::*;
//!
//! #[derive(Debug, Error)]
//! enum AppError {
//!     #[error_from("App IO: {}", 0)]
//!     Io(std::io::Error),
//! }
//!
//! type Result<T> = std::result::Result<T, AppError>;
//!
//! fn example() -> Result<()> {
//!     let _file = std::fs::File::open("not-found.txt")?;
//!     unreachable!()
//! }
//!
//! let error = example().unwrap_err();
//! assert_eq!(error.to_string().as_str(),
//!     "App IO: No such file or directory (os error 2)");
//! ```
//!
//! ## Custom error kind
//!
//! `#[error_kind]` attribute describes custom error kind.
//! Could be defined without fields or with fields tuple.
//!
//! ```rust
//! use error_rules::*;
//!
//! #[derive(Debug, Error)]
//! enum AppError {
//!     #[error_kind("App: error without arguments")]
//!     E1,
//!     #[error_kind("App: code:{} message:{}", 0, 1)]
//!     E2(usize, String),
//! }
//!
//! type Result<T> = std::result::Result<T, AppError>;
//!
//! fn example_1() -> Result<()> {
//!     Err(AppError::E1)
//! }
//!
//! fn example_2() -> Result<()> {
//!     Err(AppError::E2(404, "Not Found".to_owned()))
//! }
//!
//! let error = example_1().unwrap_err();
//! assert_eq!(error.to_string().as_str(),
//!     "App: error without arguments");
//!
//! let error = example_2().unwrap_err();
//! assert_eq!(error.to_string().as_str(),
//!     "App: code:404 message:Not Found");
//! ```
//!
//! ## Display attributes
//!
//! `#[error_from]` and `#[error_kind]` contain list of attributes to display error.
//! First attribute should be literal string. Other attributes is a number of the
//! unnamed field in the tuple. Started from 0.
//!
//! ## Error chain
//!
//! By implementing error for nested modules the primary error handler returns full chain of the error.
//!
//! ```rust
//! use error_rules::*;
//!
//! #[derive(Debug, Error)]
//! enum ModError {
//!     #[error_from("Mod IO: {}", 0)]
//!     Io(std::io::Error),
//! }
//!
//! fn mod_example() -> Result<(), ModError> {
//!     let _file = std::fs::File::open("not-found.txt")?;
//!     unreachable!()
//! }
//!
//! #[derive(Debug, Error)]
//! enum AppError {
//!     #[error_from("App: {}", 0)]
//!     Mod(ModError),
//! }
//!
//! fn app_example() -> Result<(), AppError> {
//!     mod_example()?;
//!     unreachable!()
//! }
//!
//! let error = app_example().unwrap_err();
//! assert_eq!(error.to_string().as_str(),
//!     "App: Mod IO: No such file or directory (os error 2)");
//! ```

extern crate proc_macro;

use proc_macro2::{TokenStream, Span, Ident};
use quote::quote;
use syn::{
    self,
    parse_macro_input,
};


#[proc_macro_derive(Error, attributes(error_from, error_kind))]
pub fn error_rules_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);


    if let syn::Data::Enum(ref s) = input.data {
        impl_error_rules_derive(&input, s).into()
    } else {
        panic!("#[derive(Error)] only for enum")
    }
}


fn impl_display_item(meta_list: &syn::MetaList) -> TokenStream {
    let mut attr_list = TokenStream::new();

    let fmt = match &meta_list.nested[0] {
        syn::NestedMeta::Literal(syn::Lit::Str(v)) => v.value(),
        _ => panic!("first attribute shoud be literal"),
    };
    attr_list.extend(quote! { #fmt });

    for attr in meta_list.nested.iter().skip(1) {
        let attr = match attr {
            syn::NestedMeta::Literal(syn::Lit::Int(v)) => v.value(),
            _ => panic!("attributes should be number"),
        };

        let attr_id = Ident::new(&format!("i{}", attr), Span::call_site());
        attr_list.extend(quote! { , #attr_id });
    }

    quote! { write!(f, #attr_list) }
}


fn impl_error_rules_derive(input: &syn::DeriveInput, data: &syn::DataEnum) -> TokenStream {

    let enum_id = &input.ident;

    let mut from_list = TokenStream::new();
    let mut source_list = TokenStream::new();
    let mut display_list = TokenStream::new();

    #[derive(PartialEq)]
    enum AttrType {
        ErrorFrom,
        ErrorKind,
    };

    for variant in &data.variants {
        let item_id = &variant.ident;
        let item_id = quote! { #enum_id::#item_id };

        for attr in &variant.attrs {
            let meta = attr.parse_meta().unwrap();

            let attr_name = meta.name().to_string();
            let attr_type = match attr_name.as_str() {
                "error_from" => AttrType::ErrorFrom,
                "error_kind" => AttrType::ErrorKind,
                _ => continue,
            };

            let meta_list = match meta {
                syn::Meta::List(v) => v,
                _ => panic!("#[{}] meta format mismatch", attr_name),
            };

            if meta_list.nested.is_empty() {
                panic!("#[{}] should have one or more attributes", attr_name)
            }

            let mut ident_list = TokenStream::new();

            match &variant.fields {
                syn::Fields::Unit if attr_type == AttrType::ErrorKind => {}
                syn::Fields::Unnamed(fields) if attr_type == AttrType::ErrorKind => {
                    for i in 0 .. fields.unnamed.len() {
                        let field_id = Ident::new(&format!("i{}", i), Span::call_site());
                        ident_list.extend(quote! { #field_id, });
                    }
                }
                syn::Fields::Unnamed(fields) if attr_type == AttrType::ErrorFrom => {
                    if fields.unnamed.len() != 1 {
                        panic!("#[{}] varian should contain one field", attr_name)
                    }
                    ident_list.extend(quote! { i0 });
                    let field = &fields.unnamed[0];
                    let ty = &field.ty;
                    from_list.extend(quote! {
                        impl From<#ty> for #enum_id {
                            #[inline]
                            fn from(e: #ty) -> #enum_id { #item_id ( e ) }
                        }
                    });
                    source_list.extend(quote! {
                        #item_id (i0) => Some(i0),
                    });
                }
                _ => panic!("#[{}] field format mismatch", attr_name),
            };

            let w = impl_display_item(&meta_list);
            if ident_list.is_empty() {
                display_list.extend(quote! {
                    #item_id => #w,
                });
            } else {
                display_list.extend(quote! {
                    #item_id ( #ident_list ) => #w,
                });
            }
        }
    }

    let expanded = quote! {
        impl std::fmt::Display for #enum_id {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    #display_list
                }
            }
        }

        impl std::error::Error for #enum_id {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    #source_list
                    _ => None,
                }
            }
        }

        #from_list
    };

    expanded
}
