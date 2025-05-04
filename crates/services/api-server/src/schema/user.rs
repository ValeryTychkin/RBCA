use std::str::FromStr;

use repository_db_lib::user::user_entity;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use strum_macros::{Display, EnumString, IntoStaticStr};
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
pub enum StaffPermission {
    CreateApplication,

    CreateStaffUser,
    DeleteStaffUser,
    UpdateStaffUser,

    DeleteUser,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<StaffPermission>>,
    #[serde(with = "rfc3339")]
    #[schemars(schema_with = "date_time_rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    #[schemars(schema_with = "date_time_rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl User {
    pub fn from_model(model: &user_entity::Model) -> Self {
        let mut staff_permissions = Vec::<StaffPermission>::new();
        for permission in &model.staff_permissions {
            staff_permissions.push(StaffPermission::from_str(&permission.to_string()).unwrap());
        }
        let permissions = match model.is_staff {
            true => Some(staff_permissions),
            false => None,
        };
        Self {
            id: model.id,
            name: model.name.to_owned(),
            email: model.email.to_owned(),
            permissions,
            updated_at: model.updated_at,
            created_at: model.created_at,
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct UserList {
    pub users: Vec<User>,
    #[serde(flatten)]
    pub pagination: Pagination,
}

impl UserList {
    pub fn from_models(
        models: &Vec<user_entity::Model>,
        limit: i64,
        offset: u64,
        total: u64,
    ) -> Self {
        let mut users = Vec::<User>::new();
        for model in models {
            users.push(User::from_model(model));
        }
        Self {
            users,
            pagination: Pagination {
                limit,
                offset,
                total,
            },
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct UpdateUserPassword {
    #[serde(deserialize_with = "string_1_255")]
    pub old_password: String,
    #[serde(deserialize_with = "string_1_255")]
    pub new_password: String,
}
