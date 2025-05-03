use sea_orm::entity::{prelude::*, ActiveValue};
use strum_macros::{Display, EnumString, IntoStaticStr};
use uuid::Uuid;

use time::OffsetDateTime;

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, EnumString, IntoStaticStr, Display,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::N(255))",
    rename_all = "PascalCase"
)]
pub enum AppStaffPermissions {
    UpdateApplication,
    ReadApplication,
    DeleteApplication,

    CreateKey,
    ReadKey,
    ReadKeyDetail,
    UpdateKey,
    DeleteKey,
}

impl AppStaffPermissions {
    pub fn get_all() -> Vec<Self> {
        let mut result = Self::get_all_application();
        result.append(&mut Self::get_all_key());
        result
    }

    pub fn get_all_application() -> Vec<Self> {
        vec![
            Self::UpdateApplication,
            Self::ReadApplication,
            Self::DeleteApplication,
        ]
    }

    pub fn get_all_key() -> Vec<Self> {
        vec![
            Self::CreateKey,
            Self::ReadKey,
            Self::ReadKeyDetail,
            Self::UpdateKey,
            Self::DeleteKey,
        ]
    }
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "app_staff")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub application_id: Uuid,
    pub user_id: Uuid,
    pub permissions: Vec<AppStaffPermissions>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::application::Entity",
        from = "Column::ApplicationId",
        to = "super::application::Column::Id"
    )]
    Application,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

// `Related` trait has to be implemented by hand
impl Related<super::application::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Application.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut s = self;

        if insert {
            s.id = ActiveValue::set(Uuid::new_v4());
        }
        s.updated_at = ActiveValue::set(OffsetDateTime::now_utc());
        Ok(s)
    }
}
