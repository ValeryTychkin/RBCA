use super::base::Pagination;
use rocket::serde::{uuid::Uuid, Deserialize, Serialize};
use rocket_okapi::JsonSchema;
use time::{serde::rfc3339, OffsetDateTime};
use util_lib::{date::schema::date_time_rfc3339, string::validate::string_1_255};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Organization {
    pub id: Uuid,
    pub display_name: String,
    pub is_deleted: bool,
    #[serde(with = "rfc3339")]
    #[schemars(schema_with = "date_time_rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct OrganizationCreate {
    #[serde(deserialize_with = "string_1_255")]
    pub display_name: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct OrganizationList {
    pub organizations: Vec<Organization>,
    #[serde(flatten)]
    pub pagination: Pagination,
}
