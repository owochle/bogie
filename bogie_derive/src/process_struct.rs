use darling::{FromField, FromMeta};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{DataStruct, Fields, FieldsNamed, FieldsUnnamed, Index, Visibility};
use crate::attributes::{FieldOptions, Formatter, GlobalOptions};

fn process_named(fields: &FieldsNamed, global_options: &GlobalOptions) -> darling::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    fields.named.iter().filter_map(|field| {
        let opts = match FieldOptions::from_field(&field) {
            Ok(o) => o.options,
            Err(e) => return Some(Err(e))
        };

        if opts.skip {
            return None
        }

        if let Visibility::Public(_) = field.vis {

        } else if global_options.pub_only {
            return None
        }

        let opts = opts.into_formatter();

        let name = field.ident.clone().expect("No ident on named struct field");
        let fmt = opts.unwrap_or_else(|| global_options.options.clone().into_formatter().unwrap_or(Formatter::Debug)).as_dyn_debug(quote! {
            self.#name
        });

        Some(Ok((name.into_token_stream(), fmt)))
    }).collect()
}

fn process_unnamed(fields: &FieldsUnnamed, global_options: &GlobalOptions) -> darling::Result<(Vec<TokenStream>, Vec<TokenStream>)> {
    fields.unnamed.iter().enumerate().filter_map(|(index, field)| {
        let opts = match FieldOptions::from_field(&field) {
            Ok(o) => o.options,
            Err(e) => return Some(Err(e))
        };

        if opts.skip {
            return None
        }

        if let Visibility::Public(_) = field.vis {

        } else if global_options.pub_only {
            return None
        }

        let opts = opts.into_formatter();

        let name = Index::from(index);
        let fmt = opts.unwrap_or_else(|| global_options.options.clone().into_formatter().unwrap_or(Formatter::Debug)).as_dyn_debug(quote! {
            self.#name
        });

        Some(Ok((name.into_token_stream(), fmt)))
    }).collect()
}

pub fn process_struct(name: &Ident, data: &DataStruct, global_options: &GlobalOptions) -> darling::Result<TokenStream> {
    if data.fields.is_empty() {
        return Ok(quote! {
            ::core::fmt::Formatter::write_str(f, stringify!(#name))
        })
    }

    let (idents, formatters) = match &data.fields {
        Fields::Named(named) => {
            process_named(named, global_options)
        }
        Fields::Unnamed(unnamed) => {
            process_unnamed(unnamed, global_options)
        }
        Fields::Unit => unreachable!()
    }?;

    let is_tuple = match data.fields {
        Fields::Unnamed(_) => true,
        _ => false
    };

    match idents.len() {
        0 => unreachable!(),
        v if (1..=5).contains(&v) => {
            let function_ident = if is_tuple {
                Ident::from_string(&format!("debug_tuple_field{:?}_finish", v))?
            } else {
                Ident::from_string(&format!("debug_struct_field{:?}_finish", v))?
            };

            let fmt_iter = formatters.iter();

            if is_tuple {
                Ok(quote! {
                    bogie::FormatterExt::#function_ident(
                        f,
                        stringify!(#name),
                        #(#fmt_iter),*
                    )
                })
            } else {
                let ident_iter = idents.iter();
                Ok(quote! {
                    bogie::FormatterExt::#function_ident(
                        f,
                        stringify!(#name),
                        #(stringify!(#ident_iter), #fmt_iter),*
                    )
                })
            }
        }
        _ => {
            let fmt_iter = formatters.iter();

            if is_tuple {
                Ok(quote! {
                    bogie::FormatterExt::debug_tuple_fields_finish(
                        f,
                        stringify!(#name),
                        &[#(#fmt_iter),*]
                    )
                })
            } else {
                let ident_iter = idents.iter();
                Ok(quote! {
                    bogie::FormatterExt::debug_struct_fields_finish(
                        f,
                        stringify!(#name),
                        &[#(stringify!(#ident_iter)),*],
                        &[#(#fmt_iter),*]
                    )
                })
            }
        }
    }
}