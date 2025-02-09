pub use util_lib_proc::guard_permission;

#[derive(Clone)]
pub struct Guard<T>(Option<T>);
