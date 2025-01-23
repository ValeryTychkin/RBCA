pub mod schema;
use std::str::FromStr;

/// A wrapper around `time::OffsetDateTime` to provide custom formatting and deserialization for RFC3339.
///
/// This struct is used for handling date and time values in the `OffsetDateTime` format (RFC3339).
/// It provides functionality for serialization, deserialization, and conversion to and from `time::OffsetDateTime`.
/// The main purpose is to support forms and JSON serialization/deserialization in web frameworks like Rocket and Serde.
///
/// # Example
/// ```rust
/// // Deserialize from a string representation of an RFC3339 datetime
/// let dt: OffsetDateTimeForm = serde_json::from_str("\"2025-01-01T12:00:00+00:00\"").unwrap();
/// // Serialize to an RFC3339 formatted string
/// let json = serde_json::to_string(&dt).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OffsetDateTimeForm(time::OffsetDateTime);

impl OffsetDateTimeForm {
    /// Converts the `OffsetDateTimeForm` to `time::OffsetDateTime`.
    ///
    /// This method allows easy extraction of the inner `time::OffsetDateTime` value.
    ///
    /// # Example
    /// ```rust
    /// let form = OffsetDateTimeForm::from_str("2025-01-01T12:00:00+00:00").unwrap();
    /// let time = form.to_time();
    /// ```
    pub fn to_time(&self) -> time::OffsetDateTime {
        self.0
    }
}

impl<'v> rocket::form::FromFormField<'v> for OffsetDateTimeForm {
    /// Rocket form field deserialization implementation for `OffsetDateTimeForm`.
    ///
    /// Converts form input into `OffsetDateTimeForm` by parsing the value as an RFC3339 datetime string.
    ///
    /// # Example
    /// ```rust
    /// // This example assumes Rocket request handling is set up correctly:
    /// #[post("/submit", data = "<datetime>")]
    /// fn submit(datetime: OffsetDateTimeForm) { ... }
    /// ```
    fn from_value(field: rocket::form::ValueField<'v>) -> rocket::form::Result<'v, Self> {
        let dt = Self::from_str(field.value)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
        Ok(dt)
    }
}

impl FromStr for OffsetDateTimeForm {
    type Err = time::Error;

    /// Parses a string into an `OffsetDateTimeForm`.
    ///
    /// This implementation parses an RFC3339 datetime string into the `OffsetDateTimeForm` struct.
    /// It uses `time::OffsetDateTime` to ensure proper datetime formatting.
    ///
    /// # Example
    /// ```rust
    /// let form = OffsetDateTimeForm::from_str("2025-01-01T12:00:00+00:00").unwrap();
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        time::OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)
            .map(|p| OffsetDateTimeForm(p))
            .map_err(time::Error::from)
    }
}

impl serde::Serialize for OffsetDateTimeForm {
    /// Serializes `OffsetDateTimeForm` into a RFC3339 datetime string.
    ///
    /// This implementation allows `OffsetDateTimeForm` to be serialized to a JSON string in the RFC3339 format.
    ///
    /// # Example
    /// ```rust
    /// let form = OffsetDateTimeForm::from_str("2025-01-01T12:00:00+00:00").unwrap();
    /// let json = serde_json::to_string(&form).unwrap();
    /// ```
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(
            &self
                .0
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap(),
        )
    }
}

impl<'de> serde::Deserialize<'de> for OffsetDateTimeForm {
    /// Deserializes `OffsetDateTimeForm` from a RFC3339 datetime string.
    ///
    /// This implementation uses a custom visitor to parse RFC3339 strings into `OffsetDateTimeForm`.
    ///
    /// # Example
    /// ```rust
    /// let dt: OffsetDateTimeForm = serde_json::from_str("\"2025-01-01T12:00:00+00:00\"").unwrap();
    /// ```
    fn deserialize<D>(deserializer: D) -> Result<OffsetDateTimeForm, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(OffsetDateTimeVisitor)
    }
}

struct OffsetDateTimeVisitor;

impl<'de> serde::de::Visitor<'de> for OffsetDateTimeVisitor {
    type Value = OffsetDateTimeForm;

    /// Provides a description of the expected format for this visitor. This is used in error messages
    /// when the input does not match the expected format.
    ///
    /// # Returns
    /// A description string indicating that a datetime in RFC3339 format is expected.
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a datetime in RFC3339 format")
    }

    /// Called when the input value is a string (`&str`). Attempts to parse the string as a datetime
    /// in RFC3339 format and returns the resulting `OffsetDateTimeForm`. If parsing fails, a custom error is returned.
    ///
    /// # Parameters
    /// - `v`: The string slice representing the input datetime value.
    ///
    /// # Returns
    /// A result containing the parsed `OffsetDateTimeForm` if successful, or an error message.
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        OffsetDateTimeForm::from_str(v).map_err(|e| E::custom(format!("invalid datetime: {}", e)))
    }

    /// Called when the input value is a `String`. Attempts to parse the string as a datetime
    /// in RFC3339 format and returns the resulting `OffsetDateTimeForm`. If parsing fails, a custom error is returned.
    ///
    /// # Parameters
    /// - `v`: The owned string representing the input datetime value.
    ///
    /// # Returns
    /// A result containing the parsed `OffsetDateTimeForm` if successful, or an error message.
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        OffsetDateTimeForm::from_str(&v).map_err(|e| E::custom(format!("invalid datetime: {}", e)))
    }
}

impl schemars::JsonSchema for OffsetDateTimeForm {
    /// Returns the name of the schema for the `OffsetDateTimeForm`. This name will appear in
    /// the generated OpenAPI or Swagger documentation.
    fn schema_name() -> String {
        "Date Time".to_string()
    }

    /// Generates the JSON schema for the `OffsetDateTimeForm` type. This defines the format
    /// and structure of the object when it is included in the OpenAPI or Swagger documentation.
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let mut schema: schemars::schema::SchemaObject = <String>::json_schema(gen).into();
        schema.format = Some("RFC3339".to_owned());
        schema.into()
    }
}
