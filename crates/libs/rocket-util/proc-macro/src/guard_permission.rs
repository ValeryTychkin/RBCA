use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, parse_str, Expr, FnArg, Ident, ItemFn, Pat, PatIdent, PatType,
    Type, TypePath,
};
use util_lib::string::snake_to_camel;

fn guard_arg_name_default() -> Ident {
    parse_str("guard").unwrap()
}

#[derive(Debug, FromMeta)]
struct GuardPermissionArgs {
    #[darling(rename = "arg_name", default = "guard_arg_name_default")]
    guard_arg_name: Ident,
    error_ty: Ident,
    perm_error: Ident,
    #[darling(multiple)]
    any_perms: Vec<Expr>,
    #[darling(multiple)]
    all_perms: Vec<Expr>,
}

pub fn impl_guard_permission(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn = parse_macro_input!(item as ItemFn);

    // Get the function's name as a string.
    let fn_name_str = item_fn.sig.ident.to_string();

    // Parse args
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };
    let macro_args = match GuardPermissionArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let handle_ident = gen_handle_struct_ident(fn_name_str.as_str());

    let guard_ident =
        match add_handle_guard_with_hide(&mut item_fn, &handle_ident, &macro_args.guard_arg_name) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(e.write_errors());
            }
        };

    let handle_def = gen_handle_guard_struct(
        &handle_ident,
        &guard_ident,
        &macro_args.error_ty,
        &macro_args.perm_error,
        &macro_args.any_perms,
        &macro_args.all_perms,
    );

    // Combine the struct definition with the modified function.
    let output = quote! {
        #handle_def

        #[allow(dead_code)]
        #item_fn
    };

    output.into()
}

fn update_arg_type(item_fn: &mut ItemFn, arg_ident: &Ident, new_type: &Ident) -> Option<Ident> {
    let mut arg_old_type_opt: Option<Ident> = None;
    let mut arg_idx: usize = 0;
    for (idx, input) in item_fn.sig.inputs.iter().enumerate() {
        if let FnArg::Typed(PatType { pat, ty, .. }) = input {
            if let Pat::Ident(PatIdent { ident, .. }) = pat.as_ref() {
                if ident.to_string() == arg_ident.to_string() {
                    arg_idx = idx;
                    if let Type::Path(TypePath { path, .. }) = &**ty {
                        if let Some(last) = path.segments.last() {
                            arg_old_type_opt = Some(last.ident.clone());
                        }
                    }
                }
            }
        }
    }

    let arg_old_type = match arg_old_type_opt {
        Some(v) => v,
        None => {
            return None;
        }
    };

    item_fn.sig.inputs[arg_idx] = parse_quote!(#arg_ident: #new_type);

    Some(arg_old_type)
}

fn gen_handle_guard_struct(
    handle_ident: &Ident,
    guard_ident: &Ident,
    error_type: &Ident,
    permission_error: &Ident,
    any_perms: &Vec<Expr>,
    all_perms: &Vec<Expr>,
) -> proc_macro2::TokenStream {
    quote! {
        pub struct #handle_ident(#guard_ident);

        #[async_trait]
        impl<'r> rocket::request::FromRequest<'r> for #handle_ident {
            type Error = #error_type;

            async fn from_request(request: &'r rocket::request::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
                let guard = rocket::outcome::try_outcome!(request.guard::<#guard_ident>().await);
                let all_perms = vec![#(#all_perms),*];
                let any_perms = vec![#(#any_perms),*];
                let perms = guard.get_permissions().await;
                if all_perms.len() > 0 {
                    if !all_perms.iter().all(|perm| perms.contains(perm)) {
                        return rocket::outcome::Outcome::Error((rocket::http::Status::Forbidden, #error_type::#permission_error));
                    }
                }
                if any_perms.len() > 0 {
                    if !any_perms.iter().any(|perm| perms.contains(perm)) {
                        return rocket::outcome::Outcome::Error((rocket::http::Status::Forbidden, #error_type::#permission_error));
                    }
                }
                return rocket::outcome::Outcome::Success(Self(guard))
            }
        }
    }
}

fn gen_handle_struct_ident(original_fn_name: &str) -> Ident {
    let struct_name = format!("Struct{}", snake_to_camel(original_fn_name, true));
    parse_str(struct_name.as_str()).unwrap()
}

fn add_handle_guard_with_hide(
    item_fn: &mut ItemFn,
    handle_ident: &Ident,
    guard_arg_ident: &Ident,
) -> Result<Ident, Error> {
    let err: Error;
    match add_handle_guard(item_fn, handle_ident, guard_arg_ident) {
        Ok(v) => {
            return Ok(v);
        }
        Err(e) => err = e,
    };

    // Retry found hiden argument
    let new_guard_arg_ident =
        parse_str(format!("_{}", guard_arg_ident.to_string()).as_str()).unwrap();
    match add_handle_guard(item_fn, handle_ident, &new_guard_arg_ident) {
        Ok(v) => Ok(v),
        Err(_) => Err(err), // If retry result with hiden was an error, return error from first try
    }
}

fn add_handle_guard(
    item_fn: &mut ItemFn,
    handle_ident: &Ident,
    guard_arg_ident: &Ident,
) -> Result<Ident, Error> {
    let guard_ty = match update_arg_type(item_fn, guard_arg_ident, handle_ident) {
        Some(v) => v,
        None => {
            return Err(Error::custom(format!(
                "Argument {} not found",
                guard_arg_ident.to_string()
            )));
        }
    };
    add_unpack_handle_guard(item_fn, guard_arg_ident);
    Ok(guard_ty)
}

fn add_unpack_handle_guard(item_fn: &mut ItemFn, guard_ident: &Ident) {
    let original_block = &item_fn.block;
    item_fn.block = syn::parse_quote!({
        let #guard_ident = #guard_ident.0;
        #original_block
    });
}
