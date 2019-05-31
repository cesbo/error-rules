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


fn impl_display_item(meta: &syn::Meta) -> TokenStream {
    let meta_list = match meta {
        syn::Meta::List(v) => v,
        _ => panic!("display() format mismatch"),
    };

    if meta_list.nested.is_empty() {
        panic!("display() should have one or more attributes")
    }

    let mut attr_list: Vec<TokenStream> = Vec::new();

    let fmt = match &meta_list.nested[0] {
        syn::NestedMeta::Literal(syn::Lit::Str(v)) => v.value(),
        _ => panic!("display() first attribute shoud be literal"),
    };
    attr_list.push(quote! { #fmt });

    for attr in meta_list.nested.iter().skip(1) {
        let attr = match attr {
            syn::NestedMeta::Literal(syn::Lit::Int(v)) => v.value(),
            _ => panic!("display() attributes should be number"),
        };

        let attr_id = Ident::new(&format!("i{}", attr), Span::call_site());
        attr_list.push(quote! { #attr_id });
    }

    quote! { write!(f, #( #attr_list, )*) }
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
                _ => panic!("#[{}] format mismatch", attr_name),
            };

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
                            fn from(e: #ty) -> #enum_id { #enum_id::#item_id ( e ) }
                        }
                    });
                    source_list.extend(quote! {
                        #enum_id::#item_id (i0) => Some(i0),
                    });
                }
                _ => panic!("#[{}] format mismatch", attr_name),
            };

            for item in &meta_list.nested {
                let item_meta = match item {
                    syn::NestedMeta::Meta(v) => v,
                    _ => continue,
                };

                let meta_name = item_meta.name();
                if meta_name == "display" {
                    let w = impl_display_item(&item_meta);
                    if ident_list.is_empty() {
                        display_list.extend(quote! {
                            #enum_id::#item_id => #w,
                        });
                    } else {
                        display_list.extend(quote! {
                            #enum_id::#item_id ( #ident_list ) => #w,
                        });
                    }
                }
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
