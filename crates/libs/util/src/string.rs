pub fn snake_to_camel(snake: &str, upper_first: bool) -> String {
    let mut camel = String::new();
    let mut need_upper = upper_first;

    for ch in snake.chars() {
        if ch == '_' {
            need_upper = true;
        } else {
            if need_upper {
                camel.push(ch.to_ascii_uppercase());
                need_upper = false;
            } else {
                camel.push(ch);
            }
        }
    }

    camel
}

pub fn camel_to_snake(camel: &str) -> String {
    let mut snake = String::new();

    for (i, ch) in camel.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 {
                snake.push('_');
            }
            snake.push(ch.to_ascii_lowercase());
        } else {
            snake.push(ch);
        }
    }

    snake
}

pub mod validate {
    use super::validate;
    use serde::{de::Error, Deserialize, Deserializer};

    pub fn is_str_1_255(v: &str) -> bool {
        0 < v.len() && v.len() <= 255
    }

    pub fn is_str_0_255(v: &str) -> bool {
        v.len() <= 255
    }

    pub fn is_str_1_1024(v: &str) -> bool {
        0 < v.len() && v.len() <= 1024
    }

    pub fn is_str_0_1024(v: &str) -> bool {
        v.len() <= 1024
    }

    pub fn is_str_1_2048(v: &str) -> bool {
        0 < v.len() && v.len() <= 2048
    }

    pub fn is_str_0_2048(v: &str) -> bool {
        v.len() <= 2048
    }

    /// Validates that a deserialized string has a length between 1 and 255 characters.
    ///
    /// This function uses the `validate::is_str_1_255` utility to enforce the constraint.
    /// If the string length is not within the range, it returns a custom deserialization error.
    ///
    /// # Parameters
    /// - `d`: A deserializer that provides the input to be deserialized into a String.
    ///
    /// # Returns
    /// - `Ok(String)` if the string satisfies the length constraint.
    /// - `Err(D::Error)` if the string is invalid or the deserialization fails.
    ///
    /// # Example
    /// ```
    /// #[derive(Deserialize)]
    /// struct MyStruct {
    ///     #[serde(deserialize_with = "string_1_255")]
    ///     name: String,
    /// }
    /// ```
    pub fn string_1_255<'de, D>(d: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(d) {
            Ok(v) => {
                if !validate::is_str_1_255(v.as_str()) {
                    return Err(D::Error::custom(
                        "invalid length, String must be lte 255 and gt 0",
                    ));
                }
                Ok(v)
            }
            Err(e) => Err(e),
        }
    }

    /// Validates that a deserialized string has a length between 0 and 255 characters.
    ///
    /// This function uses the `validate::is_str_0_255` utility to enforce the constraint.
    /// If the string length exceeds the range, it returns a custom deserialization error.
    ///
    /// # Parameters
    /// - `d`: A deserializer that provides the input to be deserialized into a String.
    ///
    /// # Returns
    /// - `Ok(String)` if the string satisfies the length constraint.
    /// - `Err(D::Error)` if the string is invalid or the deserialization fails.
    ///
    /// # Example
    /// ```
    /// #[derive(Deserialize)]
    /// struct MyStruct {
    ///     #[serde(deserialize_with = "string_0_255")]
    ///     description: String,
    /// }
    /// ```
    pub fn string_0_255<'de, D>(d: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(d) {
            Ok(v) => {
                if !validate::is_str_0_255(v.as_str()) {
                    return Err(D::Error::custom("invalid length, String must be lte 255"));
                }
                Ok(v)
            }
            Err(e) => Err(e),
        }
    }

    /// Validates that a deserialized string has a length between 1 and 1024 characters.
    ///
    /// This function uses the `validate::is_str_1_1024` utility to enforce the constraint.
    /// If the string length is not within the range, it returns a custom deserialization error.
    ///
    /// # Parameters
    /// - `d`: A deserializer that provides the input to be deserialized into a String.
    ///
    /// # Returns
    /// - `Ok(String)` if the string satisfies the length constraint.
    /// - `Err(D::Error)` if the string is invalid or the deserialization fails.
    ///
    /// # Example
    /// ```
    /// #[derive(Deserialize)]
    /// struct MyStruct {
    ///     #[serde(deserialize_with = "string_1_1024")]
    ///     name: String,
    /// }
    /// ```
    pub fn string_1_1024<'de, D>(d: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(d) {
            Ok(v) => {
                if !validate::is_str_1_1024(v.as_str()) {
                    return Err(D::Error::custom(
                        "invalid length, String must be lte 1024 and gt 0",
                    ));
                }
                Ok(v)
            }
            Err(e) => Err(e),
        }
    }

    /// Validates that a deserialized string has a length between 0 and 1024 characters.
    ///
    /// This function uses the `validate::is_str_0_1024` utility to enforce the constraint.
    /// If the string length is not within the range, it returns a custom deserialization error.
    ///
    /// # Parameters
    /// - `d`: A deserializer that provides the input to be deserialized into a String.
    ///
    /// # Returns
    /// - `Ok(String)` if the string satisfies the length constraint.
    /// - `Err(D::Error)` if the string is invalid or the deserialization fails.
    ///
    /// # Example
    /// ```
    /// #[derive(Deserialize)]
    /// struct MyStruct {
    ///     #[serde(deserialize_with = "string_0_1024")]
    ///     name: String,
    /// }
    /// ```
    pub fn string_0_1024<'de, D>(d: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(d) {
            Ok(v) => {
                if !validate::is_str_1_1024(v.as_str()) {
                    return Err(D::Error::custom("invalid length, String must be lte 1024"));
                }
                Ok(v)
            }
            Err(e) => Err(e),
        }
    }

    /// Validates that a deserialized string has a length between 1 and 2048 characters.
    ///
    /// This function uses the `validate::is_str_1_2048` utility to enforce the constraint.
    /// If the string length is not within the range, it returns a custom deserialization error.
    ///
    /// # Parameters
    /// - `d`: A deserializer that provides the input to be deserialized into a String.
    ///
    /// # Returns
    /// - `Ok(String)` if the string satisfies the length constraint.
    /// - `Err(D::Error)` if the string is invalid or the deserialization fails.
    ///
    /// # Example
    /// ```
    /// #[derive(Deserialize)]
    /// struct MyStruct {
    ///     #[serde(deserialize_with = "string_1_2048")]
    ///     name: String,
    /// }
    /// ```
    pub fn string_1_2048<'de, D>(d: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(d) {
            Ok(v) => {
                if !validate::is_str_1_2048(v.as_str()) {
                    return Err(D::Error::custom(
                        "invalid length, String must be lte 2048 and gt 0",
                    ));
                }
                Ok(v)
            }
            Err(e) => Err(e),
        }
    }

    /// Validates that a deserialized string has a length between 0 and 2048 characters.
    ///
    /// This function uses the `validate::is_str_0_2048` utility to enforce the constraint.
    /// If the string length is not within the range, it returns a custom deserialization error.
    ///
    /// # Parameters
    /// - `d`: A deserializer that provides the input to be deserialized into a String.
    ///
    /// # Returns
    /// - `Ok(String)` if the string satisfies the length constraint.
    /// - `Err(D::Error)` if the string is invalid or the deserialization fails.
    ///
    /// # Example
    /// ```
    /// #[derive(Deserialize)]
    /// struct MyStruct {
    ///     #[serde(deserialize_with = "string_0_2048")]
    ///     name: String,
    /// }
    /// ```
    pub fn string_0_2048<'de, D>(d: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(d) {
            Ok(v) => {
                if !validate::is_str_0_2048(v.as_str()) {
                    return Err(D::Error::custom("invalid length, String must be lte 2048"));
                }
                Ok(v)
            }
            Err(e) => Err(e),
        }
    }
}
