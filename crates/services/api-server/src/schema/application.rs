use std::str::FromStr;

use repository_db_lib::application::{app_staff_entity, application_entity};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use strum_macros::{Display, EnumString, IntoStaticStr};
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

impl Application {
    pub fn from_model(model: &application_entity::Model) -> Self {
        Self {
            id: model.id,
            name: model.name.to_owned(),
            description: model.description.to_owned(),
            updated_at: model.updated_at,
            created_at: model.created_at,
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ApplicationList {
    pub applications: Vec<Application>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

impl ApplicationList {
    pub fn from_models(
        models: &Vec<application_entity::Model>,
        limit: i64,
        offset: u64,
        total: u64,
    ) -> Self {
        let mut applications = Vec::<Application>::new();
        for model in models {
            applications.push(Application::from_model(model));
        }
        Self {
            applications,
            pagination: Pagination {
                limit,
                offset,
                total,
            },
        }
    }
}

#[derive(
    Debug,
    Clone,
    Deserialize,
    Serialize,
    JsonSchema,
    PartialEq,
    Eq,
    EnumString,
    IntoStaticStr,
    Display,
)]
pub enum ApplicationPermissions {
    UpdateApplication,
    ReadApplication,
    DeleteApplication,

    CreateKey,
    ReadKey,
    ReadKeyDetail,
    UpdateKey,
    DeleteKey,
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct ApplicationStaff {
    pub id: Uuid,
    pub application_id: Uuid,
    pub usser_id: Uuid,
    pub permissions: Vec<ApplicationPermissions>,
    #[serde(with = "rfc3339")]
    #[schemars(schema_with = "date_time_rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    #[schemars(schema_with = "date_time_rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl ApplicationStaff {
    pub fn from_model(model: &app_staff_entity::Model) -> Self {
        let mut permissions = Vec::<ApplicationPermissions>::new();
        for perm in &model.permissions {
            permissions.push(ApplicationPermissions::from_str(&perm.to_string()).unwrap());
        }
        Self {
            id: model.id,
            application_id: model.application_id,
            usser_id: model.user_id,
            permissions,
            updated_at: model.updated_at,
            created_at: model.created_at,
        }
    }
}
