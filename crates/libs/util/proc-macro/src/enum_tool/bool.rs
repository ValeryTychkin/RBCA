use proc_macro::TokenStream;
use proc_macro_error::*;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let enm = match &ast.data {
        syn::Data::Enum(e) => e,
        _ => abort!(ast.ident, "BoolEnum can only be used on enums."),
    };

    for v in &enm.variants {
        match v.fields {
            syn::Fields::Unit => (),
            _ => abort!(v.ident, "BoolEnum can only be used on enums with unit-like variants";
                hint = "try `enum {} {{ No, Yes }}`", ast.ident
            ),
        }
    }

    let mut vnames = enum_variant_names(&enm);
    vnames.sort();
    let (no, yes) = match as_strs(&vnames).as_slice() {
        &["No", "Yes"] => ("No", "Yes"),
        &["False", "True"] => ("False", "True"),
        _ => abort!(
            ast.ident,
            "BoolEnum can only be used on enums with two variants named No and Yes, or False and True.";
            hint = "try `enum {} {{ No, Yes }}`", ast.ident
        ),
    };
    let no = syn::Ident::new(no, ast.ident.span());
    let yes = syn::Ident::new(yes, ast.ident.span());

    let name = &ast.ident;
    let out = quote! {
        impl ::core::convert::From<bool> for #name {
                fn from(b: bool) -> Self {
            if b { Self::#yes } else { Self::#no }
                }
        }
        impl ::core::convert::Into<bool> for #name {
            fn into(self) -> bool {
            match self {
                Self::#no => false,
                Self::#yes => true,
            }
            }
        }
        impl ::core::ops::Not for #name {
            type Output = Self;

            fn not(self) -> Self {
            match self {
                Self::#no => Self::#yes,
                Self::#yes => Self::#no,
            }
            }
        }
    };
    out.into()
}

pub fn as_strs<T: AsRef<str>>(v: &[T]) -> Vec<&str> {
    v.iter().map(|s| s.as_ref()).collect::<Vec<&str>>()
}

pub fn enum_variant_names(enm: &syn::DataEnum) -> Vec<String> {
    enm.variants
        .iter()
        .map(|v| v.ident.to_string())
        .collect::<Vec<String>>()
}
