pub mod prelude;

pub const OFFSET_DEFAULT: u64 = 0;
pub const LIMIT_DEFAULT: u64 = 10;
pub const LIMIT_MIN: u64 = 1;
pub const LIMIT_MAX: u64 = 9999;

pub fn offset_default() -> u64 {
    OFFSET_DEFAULT
}

pub fn option_offset_default() -> Option<u64> {
    Some(OFFSET_DEFAULT)
}

pub fn limit_default() -> u64 {
    LIMIT_DEFAULT
}

pub fn option_limit_default() -> Option<u64> {
    Some(LIMIT_DEFAULT)
}

pub fn limit_max() -> u64 {
    LIMIT_MAX
}

pub fn option_limit_max() -> Option<u64> {
    Some(LIMIT_MAX)
}

pub fn is_valid_optional_limit(limit: &Option<u64>, none_is_valid: bool) -> bool {
    match limit {
        Some(v) => (&LIMIT_MIN..=&LIMIT_MAX).contains(&v),
        None => none_is_valid,
    }
}

pub fn is_valid_limit(limit: &u64) -> bool {
    (&LIMIT_MIN..=&LIMIT_MAX).contains(&limit)
}
