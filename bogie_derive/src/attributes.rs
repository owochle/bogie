use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Path};

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(bogie))]
pub struct GlobalOptions {
    #[darling(default)]
    pub pub_only: bool,
    #[darling(default)]
    pub enum_prefix: bool,
    #[darling(flatten)]
    pub options: Options
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(bogie))]
pub struct FieldOptions {
    #[darling(flatten)]
    pub options: Options
}

#[derive(Debug, Clone, FromMeta)]
#[darling(and_then = Options::check)]
pub struct Options {
    #[darling(default)]
    pub skip: bool,
    #[darling(rename = "dbg", default)]
    pub debug: bool,
    #[darling(rename = "display", default)]
    pub display: bool,
    #[darling(rename = "Hex", default)]
    pub upper_hex: bool,
    #[darling(rename = "hex", default)]
    pub lower_hex: bool,
    #[darling(rename = "bin", default)]
    pub binary: bool,
    #[darling(rename = "empty", default)]
    pub empty: bool,
    #[darling(rename = "fn", default)]
    pub fn_path: Option<Path>
}

impl Options {
    fn sum(&self) -> usize {
        self.skip as usize + self.debug as usize + self.display as usize + self.upper_hex as usize + self.lower_hex as usize + self.binary as usize + self.empty as usize + self.fn_path.is_some() as usize
    }

    fn check(self) -> darling::Result<Self> {
        if self.sum() > 1 {
            Err(darling::Error::too_many_items(1))
        } else {
            Ok(self)
        }
    }

    pub fn into_formatter(self) -> Option<Formatter> {
        if self.skip {
            unreachable!()
        } else if self.debug {
            Some(Formatter::Debug)
        } else if self.display {
            Some(Formatter::Display)
        } else if self.upper_hex {
            Some(Formatter::UpperHex)
        } else if self.lower_hex {
            Some(Formatter::LowerHex)
        } else if self.binary {
            Some(Formatter::Binary)
        } else if self.empty {
            Some(Formatter::Empty)
        } else if let Some(p) = self.fn_path {
            Some(Formatter::FnPath(p))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub enum Formatter {
    Debug,
    Display,
    UpperHex,
    LowerHex,
    Binary,
    Empty,
    FnPath(Path)
}


impl Formatter {
    pub fn as_dyn_debug(&self, field_tokens: TokenStream) -> TokenStream {
        match self {
            Formatter::Debug => {
                quote! {
                    &#field_tokens
                }
            }
            Formatter::Display => {
                quote! {
                    &bogie::FnFormat(|f| std::fmt::Display::fmt(&#field_tokens, f))
                }
            }
            Formatter::UpperHex => {
                quote! {
                    &bogie::FnFormat(|f| std::fmt::UpperHex::fmt(&#field_tokens, f))
                }
            }
            Formatter::LowerHex => {
                quote! {
                    &bogie::FnFormat(|f| std::fmt::LowerHex::fmt(&#field_tokens, f))
                }
            }
            Formatter::Binary => {
                quote! {
                    &bogie::FnFormat(|f| std::fmt::Binary::fmt(&#field_tokens, f))
                }
            }
            Formatter::Empty => {
                quote! {
                    &()
                }
            }
            Formatter::FnPath(p) => {
                quote! {
                    &bogie::FnFormat(|f| #p(&#field_tokens, f))
                }
            }
        }
    }
}