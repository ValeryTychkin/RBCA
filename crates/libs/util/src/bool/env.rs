use std::str::FromStr;

use super::BoolEnum;

pub enum BoolError {
    ParseFromStr,
}

/// Represents a boolean value as an enum with `True` and `False` variants.
///
/// This enum provides a representation of boolean values, using `True` and `False` as its variants.
/// It implements `FromStr` to allow conversion from string representations (`"0"` and `"1"`) to the enum variants.
///
/// # Example Usage:
/// ```rust
/// #[derive(env_settings_derive::EnvSettings)]
/// #[env_settings(case_insensitive, delay, prefix = "CORE_")]
/// struct Core {
///     #[env_settings(default = 0)]
///     debug: Bool,
/// }
/// ```
#[derive(BoolEnum, Clone, Copy)]
pub enum Bool {
    True,
    False,
}

/// Implements the `FromStr` trait for the `Bool` enum.
///
/// This implementation allows parsing a `Bool` enum from a string. It accepts `"0"` to represent `Bool::False`
/// and `"1"` to represent `Bool::True`. Any other input will return a `ParseFromStr` error.
impl FromStr for Bool {
    type Err = BoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::False),
            "1" => Ok(Self::True),
            _ => Err(BoolError::ParseFromStr),
        }
    }
}
