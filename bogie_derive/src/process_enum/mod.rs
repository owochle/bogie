use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataEnum, Fields, LitStr, Variant};
use syn::spanned::Spanned;
use crate::attributes::GlobalOptions;
use crate::process_enum::named::process_named;
use crate::process_enum::unnamed::process_unnamed;

type Destructure = TokenStream;
type PrintedName = Ident;
type FormattedField = TokenStream;

pub mod named;
mod unnamed;

pub fn process_variant(type_name: &Ident, variant: &Variant, global_options: &GlobalOptions) -> darling::Result<TokenStream> {
    let variant_ident = &variant.ident;

    let variant_name = if global_options.enum_prefix {
        let str = LitStr::new(&format!("{}::{}", type_name, variant_ident), variant.span());
        quote! {
            #str
        }
    } else {
        quote! {
            stringify!(#variant_ident)
        }
    };

    if variant.fields.is_empty() {
        return Ok(quote! {
            Self::#variant_ident => ::core::fmt::Formatter::write_str(f, #variant_name),
        })
    }

    let (destructure, names, formatters) = match &variant.fields {
        Fields::Named(named) => {
            process_named(named, global_options)?
        }
        Fields::Unnamed(unnamed) => {
            process_unnamed(unnamed, global_options)?
        }
        Fields::Unit => unreachable!()
    };

    let is_tuple = match variant.fields {
        Fields::Unnamed(_) => true,
        _ => false
    };

    let body = match destructure.len() {
        0 => unreachable!(),
        v if (1..=5).contains(&v) => {
            let function_ident = if is_tuple {
                Ident::from_string(&format!("debug_tuple_field{:?}_finish", v))?
            } else {
                Ident::from_string(&format!("debug_struct_field{:?}_finish", v))?
            };

            let fmt_iter = formatters.iter();

            if is_tuple {
                quote! {
                    bogie::FormatterExt::#function_ident(
                        f,
                        #variant_name,
                        #(#fmt_iter),*
                    )
                }
            } else {
                let ident_iter = names.iter();
                quote! {
                    bogie::FormatterExt::#function_ident(
                        f,
                        #variant_name,
                        #(stringify!(#ident_iter), #fmt_iter),*
                    )
                }
            }
        }
        _ => {
            let fmt_iter = formatters.iter();

            if is_tuple {
                quote! {
                    bogie::FormatterExt::debug_tuple_fields_finish(
                        f,
                        #variant_name,
                        &[#(#fmt_iter),*]
                    )
                }
            } else {
                let ident_iter = names.iter();
                quote! {
                    bogie::FormatterExt::debug_struct_fields_finish(
                        f,
                        #variant_name,
                        &[#(stringify!(#ident_iter)),*],
                        &[#(#fmt_iter),*]
                    )
                }
            }
        }
    };

    if is_tuple {
        Ok(quote! {
            Self::#variant_ident(#(#destructure),*) => {
                #body
            }
        })
    } else {
        Ok(quote! {
            Self::#variant_ident { #(#destructure),* } => {
                #body
            }
        })
    }
}

pub fn process_enum(type_name: &Ident, data: &DataEnum, global_options: &GlobalOptions) -> darling::Result<TokenStream> {
    let variants = data.variants.iter().map(|v| {
        process_variant(type_name, v, global_options)
    }).collect::<darling::Result<Vec<TokenStream>>>()?;

    Ok(quote! {
        match self {
            #(#variants)*
        }
    })
}