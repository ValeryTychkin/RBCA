/*!
`BoolEnum` is a derive macro to create ergonomic boolean enums with less boilerplate.
It generates `From<bool>`, `Into<bool>`, and `Not` impls for your enum.

```rust
use boolenum::BoolEnum;

// Variant names can be Yes and No (in any order) ...
#[derive(BoolEnum)]
enum UseColors {
    No,
    Yes,
}

// or True and False
#[derive(BoolEnum)]
enum ShowExpired {
    True,
    False,
}

fn print_things(use_colors: UseColors, show_expired: ShowExpired) {
    if use_colors.into() { // Into<bool>
      // ...
    }
}

fn main() {
    print_things(UseColors::Yes, ShowExpired::False)
}
```

Boolean enums are useful for differentiating between boolean arguments to a function,
so you can write something like `encode(&bytes, Encrypt::Yes, Compress::No)` instead of `encode(&bytes, true, false)`.

Goes well with [structopt](https://crates.io/crates/structopt), for type-safe handling of command-line flags:

```rust
use boolenum::BoolEnum;
use structopt::StructOpt;

#[derive(BoolEnum)]
enum Verbose { No, Yes }
#[derive(BoolEnum)]
enum Colors { No, Yes }

#[derive(StructOpt)]
struct Opt {
    #[structopt(short, long, parse(from_flag))]
    verbose: Verbose, // works because Verbose implements From<bool>
    #[structopt(short, long, parse(from_flag))]
    colors: Colors,
}

fn main() {
    let opt = Opt::from_args();
    do_thing(opt.verbose, opt.colors);
}

fn do_thing(verbose: Verbose, colors: Colors) {
    if verbose.into() { }
    if colors.into() { }
}
```

`BoolEnum` works on enums with two unit variants, named either Yes and No, or True and False. The order of the variants in the enum doesn't matter.
*/

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro_error::*;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[rustfmt::skip::macros(quote)]
#[proc_macro_derive(BoolEnum)]
#[proc_macro_error]
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

fn as_strs<T: AsRef<str>>(v: &[T]) -> Vec<&str> {
    v.iter().map(|s| s.as_ref()).collect::<Vec<&str>>()
}

fn enum_variant_names(enm: &syn::DataEnum) -> Vec<String> {
    enm.variants
        .iter()
        .map(|v| v.ident.to_string())
        .collect::<Vec<String>>()
}
