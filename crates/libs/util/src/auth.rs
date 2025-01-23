use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod basic;
pub mod jwt;

/// Enum representing different token types.
///
/// This enum defines the types of tokens that can be used for authentication or authorization.
/// The two types are:
/// - `Basic`: Represents basic authentication tokens.
/// - `Bearer`: Represents bearer tokens, typically used with the `Authorization` header.
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Basic,
    Bearer,
}
