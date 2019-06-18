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
//! Converted type should implements `std::error::Error` interface.
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
//! `#[error_from]` could defined without attributes it's equal to `#[error_from("{}", 0)]`
//!
//! ## Error prefix
//!
//! `#[error_prefix]` attribute should be defined before enum declaration and
//! appends prefix into error text.
//!
//! ```rust
//! use error_rules::*;
//!
//! #[derive(Debug, Error)]
//! #[error_prefx = "App"]
//! enum AppError {
//!     #[error_from]
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
//!     "App: No such file or directory (os error 2)");
//! ```
//!
//! ## Error chain
//!
//! By implementing error for nested modules the primary error handler returns full chain of the error.
//!
//! ```rust
//! use error_rules::*;
//!
//! #[derive(Debug, Error)]
//! #[error_prefix = "Mod"]
//! enum ModError {
//!     #[error_from]
//!     Io(std::io::Error),
//! }
//!
//! fn mod_example() -> Result<(), ModError> {
//!     let _file = std::fs::File::open("not-found.txt")?;
//!     unreachable!()
//! }
//!
//! #[derive(Debug, Error)]
//! #[error_prefix = "App"]
//! enum AppError {
//!     #[error_from]
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
//!     "App: Mod: No such file or directory (os error 2)");
//! ```

extern crate proc_macro;

use proc_macro2::{TokenStream, Span, Ident};
use quote::quote;
use syn::{
    self,
    parse_macro_input,
};


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

    attr_list
}


struct ErrorRules {
    enum_id: Ident,
    prefix: String,
    from_list: TokenStream,
    source_list: TokenStream,
    display_list: TokenStream,
}


impl ErrorRules {
    fn new(ident: &Ident) -> ErrorRules {
        ErrorRules {
            enum_id: ident.clone(),
            prefix: String::default(),
            from_list: TokenStream::default(),
            source_list: TokenStream::default(),
            display_list: TokenStream::default(),
        }
    }

    fn impl_error_from_fields(&mut self,
        item_id: &TokenStream,
        variant: &syn::Variant)
    {
        let enum_id = &self.enum_id;

        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    panic!("variant should contain one field")
                }
                let field = &fields.unnamed[0];
                let ty = &field.ty;
                self.from_list.extend(quote! {
                    impl From<#ty> for #enum_id {
                        #[inline]
                        fn from(e: #ty) -> #enum_id { #item_id ( e ) }
                    }
                });
                self.source_list.extend(quote! {
                    #item_id (i0) => Some(i0),
                });
            }
            _ => panic!("field format mismatch"),
        };
    }

    fn impl_error_from_word(&mut self,
        item_id: &TokenStream,
        variant: &syn::Variant)
    {
        self.impl_error_from_fields(&item_id, variant);

        self.display_list.extend(quote! {
            #item_id ( i0 ) => write!(f, "{}", i0),
        });
    }

    fn impl_error_from_list(&mut self,
        item_id: &TokenStream,
        variant: &syn::Variant,
        meta_list: &syn::MetaList)
    {
        if meta_list.nested.is_empty() {
            self.impl_error_from_word(item_id, variant);
            return
        }

        self.impl_error_from_fields(item_id, variant);

        let w = impl_display_item(meta_list);
        self.display_list.extend(quote! {
            #item_id ( i0 ) => write!(f, #w),
        });
    }

    fn impl_error_from(&mut self,
        item_id: &TokenStream,
        variant: &syn::Variant,
        meta: &syn::Meta)
    {
        match meta {
            syn::Meta::Word(_) => self.impl_error_from_word(item_id, variant),
            syn::Meta::List(v) => self.impl_error_from_list(item_id, variant, v),
            _ => panic!("meta format mismatch"),
        }
    }

    fn impl_error_kind_list(&mut self,
        item_id: &TokenStream,
        variant: &syn::Variant,
        meta_list: &syn::MetaList)
    {
        if meta_list.nested.is_empty() {
            panic!("meta format mismatch")
        }

        match &variant.fields {
            syn::Fields::Unit => {
                let w = impl_display_item(meta_list);
                self.display_list.extend(quote! {
                    #item_id => write!(f, #w),
                });
            }
            syn::Fields::Unnamed(fields) => {
                let mut ident_list = TokenStream::new();
                for i in 0 .. fields.unnamed.len() {
                    let field_id = Ident::new(&format!("i{}", i), Span::call_site());
                    ident_list.extend(quote! { #field_id, });
                }

                let w = impl_display_item(meta_list);
                self.display_list.extend(quote! {
                    #item_id ( #ident_list ) => write!(f, #w),
                });
            }
            _ => panic!("field format mismatch"),
        };
    }

    fn impl_error_kind(&mut self,
        item_id: &TokenStream,
        variant: &syn::Variant,
        meta: &syn::Meta)
    {
        match meta {
            syn::Meta::List(v) => self.impl_error_kind_list(item_id, variant, v),
            _ => panic!("meta format mismatch"),
        }
    }

    fn impl_variant(&mut self, variant: &syn::Variant) {
        let enum_id = &self.enum_id;
        let item_id = &variant.ident;
        let item_id = quote! { #enum_id::#item_id };

        for attr in variant.attrs.iter().filter(|v| v.path.segments.len() == 1) {
            match attr.path.segments[0].ident.to_string().as_str() {
                "error_from" => {
                    let meta = attr.parse_meta().unwrap();
                    self.impl_error_from(&item_id, variant, &meta);
                    break
                }
                "error_kind" => {
                    let meta = attr.parse_meta().unwrap();
                    self.impl_error_kind(&item_id, variant, &meta);
                    break
                }
                _ => {},
            }
        }
    }

    fn build(&mut self, data: &syn::DataEnum) -> TokenStream {
        for variant in &data.variants {
            self.impl_variant(variant);
        }

        let enum_id = &self.enum_id;
        let display_list = &self.display_list;
        let source_list = &self.source_list;
        let from_list = &self.from_list;

        let mut display_prefix = TokenStream::new();
        if ! self.prefix.is_empty() {
            let prefix = &self.prefix;
            display_prefix.extend(quote! {
                write!(f, "{}: ", #prefix)?;
            });
        }

        quote! {
            impl std::fmt::Display for #enum_id {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    #display_prefix
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
        }
    }

    fn set_attrs(&mut self, attrs: &Vec<syn::Attribute>) {
        for attr in attrs.iter().filter(|v| v.path.segments.len() == 1) {
            match attr.path.segments[0].ident.to_string().as_str() {
                "error_prefix" => {
                    if let syn::Meta::NameValue(v) = &attr.parse_meta().unwrap() {
                        if let syn::Lit::Str(v) = &v.lit {
                            self.prefix = v.value();
                            break
                        }
                    }
                    panic!("meta format mismatch")
                }
                _ => {},
            }
        }
    }
}


#[proc_macro_derive(Error, attributes(error_from, error_kind, error_prefix))]
pub fn error_rules_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    if let syn::Data::Enum(ref s) = input.data {
        let mut error_rules = ErrorRules::new(&input.ident);
        error_rules.set_attrs(&input.attrs);
        error_rules.build(s).into()
    } else {
        panic!("enum required")
    }
}
