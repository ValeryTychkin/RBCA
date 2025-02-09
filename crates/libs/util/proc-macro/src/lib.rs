mod enum_tool;
mod guard_permission;

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro_error::*;

use enum_tool::bool as enum_bool;
use guard_permission::impl_guard_permission;

#[proc_macro_attribute]
pub fn guard_permission(attr: TokenStream, item: TokenStream) -> TokenStream {
    impl_guard_permission(attr, item)
}

/*
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
#[rustfmt::skip::macros(quote)]
#[proc_macro_derive(BoolEnum)]
#[proc_macro_error]
pub fn enum_bool(input: TokenStream) -> TokenStream {
    enum_bool::derive(input)
}
