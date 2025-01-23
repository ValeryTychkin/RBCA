use orm_addons_lib::prelude::*;
use rocket::form::FromForm;
use rocket_okapi::JsonSchema;
use uuid::Uuid;

use orm_addons_lib::{LIMIT_DEFAULT, OFFSET_DEFAULT};
use util_lib::date::OffsetDateTimeForm;

#[derive(JsonSchema, FromForm, EntityFilterable)]
pub struct Organization {
    pub id: Option<Uuid>,
    #[filter(rule = "like")]
    pub display_name: Option<String>,
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
