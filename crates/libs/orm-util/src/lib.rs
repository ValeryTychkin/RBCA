use std::u64;

pub mod prelude;

pub const OFFSET_DEFAULT: u64 = 0;
pub const LIMIT_DEFAULT: i64 = 10;
pub const LIMIT_MIN: i64 = -1;
pub const LIMIT_MAX: i64 = 9999;

pub fn offset_default() -> u64 {
    OFFSET_DEFAULT
}

pub fn option_offset_default() -> Option<u64> {
    Some(OFFSET_DEFAULT)
}

pub fn limit_default() -> i64 {
    LIMIT_DEFAULT
}

pub fn option_limit_default() -> Option<i64> {
    Some(LIMIT_DEFAULT)
}

pub fn limit_max() -> i64 {
    LIMIT_MAX
}

pub fn option_limit_max() -> Option<i64> {
    Some(LIMIT_MAX)
}

pub fn is_valid_optional_limit(limit: &Option<i64>, none_is_valid: bool) -> bool {
    match limit {
        Some(v) => (&LIMIT_MIN..=&LIMIT_MAX).contains(&v),
        None => none_is_valid,
    }
}

pub fn is_valid_limit(limit: &i64) -> bool {
    (&LIMIT_MIN..=&LIMIT_MAX).contains(&limit)
}

// Check optional offset value or None (if None set default)
pub fn get_offset(input: Option<u64>) -> u64 {
    match input {
        Some(v) => v,
        None => OFFSET_DEFAULT,
    }
}

// Check optional limit on max/min value or None (if None set default)
pub fn get_limit(input: Option<i64>) -> i64 {
    match input {
        Some(v) => {
            if v < LIMIT_MIN {
                return LIMIT_MIN;
            } else if v > LIMIT_MAX {
                return LIMIT_MAX;
            }
            return v;
        }
        None => LIMIT_DEFAULT,
    }
}
