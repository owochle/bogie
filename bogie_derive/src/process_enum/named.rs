use darling::FromField;
use quote::{format_ident, quote};
use syn::{FieldsNamed, Visibility};
use crate::attributes::{FieldOptions, Formatter, GlobalOptions};
use crate::process_enum::{Destructure, FormattedField, PrintedName};

pub fn process_named(fields: &FieldsNamed, global_options: &GlobalOptions) -> darling::Result<(Vec<Destructure>, Vec<PrintedName>, Vec<FormattedField>)> {
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
        
        let clean_ident = field.ident.as_ref().expect("No ident on named struct field");
        let name = format_ident!("__self_{}", clean_ident);
        let fmt = opts.unwrap_or_else(|| global_options.options.clone().into_formatter().unwrap_or(Formatter::Debug)).as_dyn_debug(quote! {
            #name
        });

        Some(Ok((quote! {
            #clean_ident: #name
        }, clean_ident.clone(), fmt)))
    }).collect()
}