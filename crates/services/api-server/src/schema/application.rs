use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use time::{serde::rfc3339, OffsetDateTime};
use util_lib::{
    date::schema::date_time_rfc3339,
    string::validate::{string_0_2048, string_1_255},
};
use uuid::Uuid;

use super::Pagination;

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct CreateApplication {
    #[serde(deserialize_with = "string_1_255")]
    pub name: String,
    #[serde(deserialize_with = "string_0_2048", default)]
    pub description: String,
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct UpdateApplication {
    #[serde(deserialize_with = "string_1_255")]
    pub name: String,
    #[serde(deserialize_with = "string_0_2048", default)]
    pub description: String,
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct Application {
    pub id: Uuid,
    #[serde(deserialize_with = "string_1_255")]
    pub name: String,
    #[serde(deserialize_with = "string_0_2048", default)]
    pub description: String,
    #[serde(with = "rfc3339")]
    #[schemars(schema_with = "date_time_rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    #[schemars(schema_with = "date_time_rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ApplicationList {
    pub applications: Vec<Application>,
    #[serde(flatten)]
    pub pagination: Pagination,
}
