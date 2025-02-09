mod guard_permission;

extern crate proc_macro;
use proc_macro::TokenStream;

use guard_permission::impl_guard_permission;

#[proc_macro_attribute]
pub fn guard_permission(attr: TokenStream, item: TokenStream) -> TokenStream {
    impl_guard_permission(attr, item)
}
