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


fn impl_from(enum_id: &syn::Ident, item_id: &syn::Ident, ty: &syn::Type) -> TokenStream {
    quote! {
        impl From<#ty> for #enum_id {
            #[inline]
            fn from(e: #ty) -> #enum_id { #enum_id::#item_id ( e ) }
        }
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

    for variant in &data.variants {
        let item_id = &variant.ident;

        for attr in &variant.attrs {
            let meta = attr.parse_meta().unwrap();
            let attr_name = meta.name();

            let meta_list = match meta {
                syn::Meta::List(v) => v,
                _ => panic!("#[{}] format mismatch", attr_name),
            };

            if attr_name == "error_from" {
                let fields = match &variant.fields {
                    syn::Fields::Unnamed(v) => v,
                    _ => panic!("#[error_from] unamed variant required"),
                };

                if fields.unnamed.len() != 1 {
                    panic!("#[error_from] varian should contain one field")
                }

                let field = fields.unnamed.iter().next().unwrap();

                from_list.extend(impl_from(&enum_id, &item_id, &field.ty));
                source_list.extend(quote! {
                    #enum_id::#item_id (i0) => Some(i0),
                });

                for item in &meta_list.nested {
                    let item_meta = match item {
                        syn::NestedMeta::Meta(v) => v,
                        _ => continue,
                    };

                    let meta_name = item_meta.name();
                    if meta_name == "display" {
                        let w = impl_display_item(&item_meta);
                        display_list.extend(quote! {
                            #enum_id::#item_id (i0) => #w,
                        });
                    }
                }
            } else if attr_name == "error_kind" {
                let mut ident_list = TokenStream::new();

                match &variant.fields {
                    syn::Fields::Unit => {}
                    syn::Fields::Unnamed(fields) => {
                        for (i, _field) in fields.unnamed.iter().enumerate() {
                            let field_id = Ident::new(&format!("i{}", i), Span::call_site());
                            ident_list.extend(quote! { #field_id, });
                        }
                    }
                    _ => panic!("#[error_kind] format mismatch"),
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
