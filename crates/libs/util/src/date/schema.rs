use rocket_okapi::JsonSchema;

use schemars::{
    gen::SchemaGenerator,
    schema::{Schema, SchemaObject},
};

/// Generates a JSON Schema OpenAPI or Swagger definition for a date-time value in RFC3339 format.
pub fn date_rfc3339(gen: &mut SchemaGenerator) -> Schema {
    let mut schema: SchemaObject = <String>::json_schema(gen).into();
    schema.format = Some("RFC3339".to_owned());
    schema.into()
}

/// Generates a JSON Schema OpenAPI or Swagger definition for a date-time value in RFC3339 format.
pub fn date_time_rfc3339(gen: &mut SchemaGenerator) -> Schema {
    let mut schema: SchemaObject = <String>::json_schema(gen).into();
    schema.format = Some("RFC3339".to_owned());
    schema.into()
}
