use darling::{FromDeriveInput};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Data, DeriveInput, Error};
use crate::attributes::{GlobalOptions};
use crate::process_enum::process_enum;
use crate::process_struct::process_struct;
use crate::process_union::process_union;

mod attributes;
mod process_struct;
mod process_enum;
mod process_union;

#[proc_macro_derive(Debogue, attributes(bogie))]
pub fn debogue_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let globals = match GlobalOptions::from_derive_input(&input) {
        Ok(o) => o,
        Err(err) => {
            return err.write_errors().into()
        }
    };

    if let Data::Enum(_) = input.data {

    } else if globals.enum_prefix {
        return Error::new(input.ident.span(), "`enum_prefix` can only target enums").into_compile_error().into()
    }

    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let format_body = match process_type(&name, &input.data, globals) {
        Ok(b) => b,
        Err(err) => {
            return err.write_errors().into()
        }
    };

    quote! {
        #[automatically_derived]
        impl #impl_generics core::fmt::Debug for #name #ty_generics #where_clause {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                #format_body
            }
        }
    }.into()
}

fn process_type(type_name: &Ident, data: &Data, global_options: GlobalOptions) -> darling::Result<TokenStream> {
    match data {
        Data::Struct(s) => {
            process_struct(type_name, s, &global_options)
        },
        Data::Enum(e) => {
            process_enum(type_name, e, &global_options)
        }
        Data::Union(u) => {
            process_union(type_name, u, &global_options)
        }
    }
}