use proc_macro::TokenStream;
use quote::quote;
use serde::{Deserialize, Serialize};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput, Lit, LitStr};

/// Enumeration representing different filter methods that can be applied in an entity filtering context.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum FilterMethod {
    Eq,
    Ne,
    Like,
    NotLike,
    In,
    NotIn,
    Gt,
    Lt,
    Gte,
    Lte,
}

impl FilterMethod {
    fn to_method_name(&self) -> String {
        let json_name = serde_json::to_string(self).unwrap();
        // Remove quotes and reutrn "\"eq\"" => "eq"
        json_name[1..json_name.len() - 1].to_string()
    }

    fn default() -> FilterMethod {
        FilterMethod::Eq
    }

    fn find(method_name: &str) -> Option<Self> {
        // Add quotes for correct json parce
        let method_name = format!("\"{}\"", method_name);
        match serde_json::from_str::<Self>(method_name.as_str()) {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}

/// Derives an implementation for the `EntityFilterableTrait` trait on a struct.
///
/// This macro automatically generates a `to_condition` method for a struct that filters its fields
/// based on filter annotations, mapping the field names and applying the specified filter methods.
/// It handles different types of filtering methods like equality, range, and pattern matching.
pub fn impl_entity_filterable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
    } else {
        panic!("Only support Struct")
    };

    let mut vars_quete = Vec::new();

    // Iterate over the fields to generate filtering logic for each field
    for field in fields.named.iter() {
        let field_ident = &field.ident;
        let field_name: &syn::Ident = field_ident.as_ref().unwrap();
        let name: String = field_name.to_string();
        let literal_var_str = LitStr::new(&name, field.span());
        let is_option = is_option(&field.ty);

        let mut ignore = false;
        let mut var_value_quote = quote! {};
        let mut value_prepare = quote! { v };
        let mut method = FilterMethod::default();
        let mut column_name = literal_var_str;

        // Process field attributes, such as filter rules, column name, and value preparation
        for attr in field.attrs.iter() {
            if attr.path().is_ident("filter") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("ignore") {
                        ignore = true;
                    } else if meta.path.is_ident("rule") {
                        let lit = meta.value()?.parse()?;
                        if let Lit::Str(lit_str) = lit {
                            match FilterMethod::find(lit_str.value().as_str()) {
                                Some(value) => {
                                    match value {
                                        FilterMethod::Like => {
                                            var_value_quote = quote! {v = format!("%{v}%"); }
                                        }
                                        _ => var_value_quote = quote! {},
                                    }
                                    method = value;
                                }
                                None => {
                                    return Err(meta.error(format!(
                                        "Invalid rule method {:?}",
                                        lit_str.value()
                                    )));
                                }
                            }
                        }
                    } else if meta.path.is_ident("value_prepare") {
                        let lit = meta.value()?.parse()?;
                        if let Lit::Str(lit_str) = lit {
                            let value_prepare_stream: proc_macro2::TokenStream =
                                lit_str.parse().unwrap();
                            value_prepare = quote! { #value_prepare_stream };
                        }
                    } else if meta.path.is_ident("column") {
                        let lit = meta.value()?.parse()?;
                        if let Lit::Str(lit_str) = lit {
                            column_name = lit_str;
                        }
                    }
                    Ok(())
                })
                .unwrap();
            }
        }

        // If the field is not marked to be ignored, generate filter logic
        if !ignore {
            let method_ident: proc_macro2::TokenStream =
                method.to_method_name().as_str().parse().unwrap();
            let mut check_value = quote! {let v = self.#field_ident.to_owned()};
            if is_option {
                check_value = quote! {let Some(v) = self.#field_ident.to_owned()}
            }
            vars_quete.push(quote! {
                if #check_value {
                    let mut v = #value_prepare;
                    #var_value_quote
                    if let Some(column) = coluns_map.get(#column_name) {
                        condition = condition.add(column.#method_ident(v));
                    }
                }
            })
        }
    }

    // Generate the implementation of the EntityFilterableTrait trait for the struct
    let expanded = quote! {
        impl orm_util_lib::prelude::EntityFilterableTrait for #ident {
            fn to_condition<E>(&self) -> sea_orm::Condition
            where
                E: sea_orm::EntityTrait,
            {
                use sea_orm::{ColumnTrait, Iterable};
                use sea_query::Iden;

                let mut coluns_map: std::collections::HashMap<String, E::Column> =
                    std::collections::HashMap::new();
                for column in E::Column::iter() {
                    coluns_map.insert(column.to_string(), column);
                }

                let mut condition = sea_orm::Condition::all();

                #(#vars_quete)*

                condition

            }
        }
    };
    expanded.into()
}

/// Determines if a given field type is an `Option<T>`.
///
/// This function checks if the field's type is wrapped in the `Option` type, which is commonly used to represent
/// nullable fields in Rust.
pub fn is_option(field_type: &syn::Type) -> bool {
    match field_type {
        syn::Type::Path(path) => {
            path.qself.is_none()
                && path.path.leading_colon.is_none()
                && !path.path.segments.is_empty()
                && path.path.segments[0].ident == "Option"
        }
        _ => false,
    }
}
