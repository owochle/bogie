use darling::FromField;
use quote::{format_ident, quote, ToTokens};
use syn::{FieldsUnnamed, Visibility};
use crate::attributes::{FieldOptions, Formatter, GlobalOptions};
use crate::process_enum::{Destructure, FormattedField, PrintedName};

pub fn process_unnamed(fields: &FieldsUnnamed, global_options: &GlobalOptions) -> darling::Result<(Vec<Destructure>, Vec<PrintedName>, Vec<FormattedField>)> {
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

        let name = format_ident!("__self_{}", index);
        let fmt = opts.unwrap_or_else(|| global_options.options.clone().into_formatter().unwrap_or(Formatter::Debug)).as_dyn_debug(quote! {
            #name
        });

        Some(Ok((name.clone().into_token_stream(), name, fmt)))
    }).collect()
}