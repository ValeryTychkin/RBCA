use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use time::{serde::rfc3339, OffsetDateTime};
use util_lib::date::schema::date_time_rfc3339;
use uuid::Uuid;

use super::base::Pagination;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub organization_id: Uuid,
    pub is_deleted: bool,
    #[serde(with = "rfc3339")]
    #[schemars(schema_with = "date_time_rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    #[schemars(schema_with = "date_time_rfc3339")]
    pub updated_ad: OffsetDateTime,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct UserList {
    pub users: Vec<User>,
    #[serde(flatten)]
    pub pagination: Pagination,
}
