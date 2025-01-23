use orm_addons_lib::{prelude::*, LIMIT_DEFAULT, OFFSET_DEFAULT};
use uuid::Uuid;

use rocket::form::FromForm;
use schemars::JsonSchema;

use util_lib::date::OffsetDateTimeForm;

#[derive(JsonSchema, FromForm, EntityFilterable)]
pub struct User {
    id: Option<Uuid>,
    #[filter(rule = "like")]
    name: Option<String>,
    #[filter(rule = "like")]
    email: Option<String>,
    #[filter(rule = "gte", value_prepare = "v.to_time()", column = "created_at")]
    pub created_start: Option<OffsetDateTimeForm>,
    #[filter(rule = "lt", value_prepare = "v.to_time()", column = "created_at")]
    pub created_end: Option<OffsetDateTimeForm>,
    #[filter(ignore)]
    #[field(default = Some(OFFSET_DEFAULT))]
    pub offset: Option<u64>,
    #[filter(ignore)]
    #[field(default = Some(LIMIT_DEFAULT))]
    pub limit: Option<u64>,
}
