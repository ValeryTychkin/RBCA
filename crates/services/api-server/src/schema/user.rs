use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use time::{serde::rfc3339, Date, OffsetDateTime};
use util_lib::{
    date::schema::{date_rfc3339, date_time_rfc3339},
    string::validate::string_1_255,
};
use uuid::Uuid;

use super::Pagination;

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct CreateUser {
    #[serde(deserialize_with = "string_1_255")]
    pub name: String,
    #[serde(deserialize_with = "string_1_255")]
    pub email: String,
    #[serde(skip)]
    pub is_staff: Option<bool>,
    #[schemars(schema_with = "date_rfc3339")]
    pub birthday: Date,
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct UpdateUser {
    #[serde(deserialize_with = "string_1_255")]
    pub name: String,
    #[schemars(schema_with = "date_rfc3339")]
    pub birthday: Date,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(with = "rfc3339")]
    #[schemars(schema_with = "date_time_rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    #[schemars(schema_with = "date_time_rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct UserList {
    pub users: Vec<User>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct UpdateUserPassword {
    #[serde(deserialize_with = "string_1_255")]
    pub old_password: String,
    #[serde(deserialize_with = "string_1_255")]
    pub new_password: String,
}
