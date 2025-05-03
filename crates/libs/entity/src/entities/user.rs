use rand::Rng;
use sea_orm::entity::{prelude::*, ActiveValue};
use strum_macros::{Display, EnumString, IntoStaticStr};
use util_lib::crypto::{Bcrypt, Hasher};
use uuid::Uuid;

use crate::event::user as user_event;

use time::{Date, OffsetDateTime};

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, EnumString, IntoStaticStr, Display,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::N(255))",
    rename_all = "PascalCase"
)]
pub enum UserStaffPermission {
    CreateApplication,

    CreateStaffUser,
    DeleteStaffUser,
    UpdateStaffUser,

    DeleteUser,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub birthday: Date,
    pub is_staff: bool,
    pub staff_permissions: Vec<UserStaffPermission>,
    #[sea_orm(default_value = "false")]
    pub is_deleted: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Model {
    pub fn is_valid_password(&self, password: &str) -> bool {
        Bcrypt::new().verify(password, self.password.as_str())
    }

    pub fn gen_password() -> String {
        rand::rng()
            .sample_iter(rand::distr::Alphanumeric)
            .take(16)
            .map(char::from)
            .collect()
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut s = self;

        let bcrypt = Bcrypt::new();
        if insert {
            s.id = ActiveValue::set(Uuid::new_v4());
            s.password = ActiveValue::set(bcrypt.hash(s.password.unwrap().as_str()));
        } else {
            // Check password on update (save hash if update)
            if !s.password.is_unchanged() {
                s.password = ActiveValue::set(bcrypt.hash(s.password.unwrap().as_str()));
            }
        }
        s.updated_at = ActiveValue::set(OffsetDateTime::now_utc());
        user_event::create_or_update(&s, insert).await;
        Ok(s)
    }

    async fn before_delete<C>(self, _db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        user_event::delete(&self).await;
        Ok(self)
    }
}
