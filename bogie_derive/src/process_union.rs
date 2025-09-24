use darling::{FromField, FromMeta};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{DataUnion, FieldsNamed, Visibility};
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

pub fn process_union(name: &Ident, data: &DataUnion, global_options: &GlobalOptions) -> darling::Result<TokenStream> {
    if data.fields.named.is_empty() {
        return Ok(quote! {
            ::core::fmt::Formatter::write_str(f, stringify!(#name))
        })
    }

    let (idents, formatters) = process_named(&data.fields, global_options)?;

    match idents.len() {
        0 => unreachable!(),
        v if (1..=5).contains(&v) => {
            let function_ident = Ident::from_string(&format!("debug_struct_field{:?}_finish", v))?;

            let fmt_iter = formatters.iter();

            let ident_iter = idents.iter();
            Ok(quote! {
                bogie::FormatterExt::#function_ident(
                    f,
                    stringify!(#name),
                    #(stringify!(#ident_iter), #fmt_iter),*
                )
            })
        }
        _ => {
            let fmt_iter = formatters.iter();

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