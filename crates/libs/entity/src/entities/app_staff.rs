use sea_orm::entity::{prelude::*, ActiveValue};
use uuid::Uuid;

use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "app_staff_permission"
)]
pub enum AppStaffPermissions {
    #[sea_orm(string_value = "UpdateApplication")]
    UpdateApplication,
    #[sea_orm(string_value = "ReadApplication")]
    ReadApplication,
    #[sea_orm(string_value = "DeleteApplication")]
    DeleteApplication,

    #[sea_orm(string_value = "CreateKey")]
    CreateKey,
    #[sea_orm(string_value = "ReadKey")]
    ReadKey,
    #[sea_orm(string_value = "ReadKeyDetail")]
    ReadKeyDetail,
    #[sea_orm(string_value = "UpdateKey")]
    UpdateKey,
    #[sea_orm(string_value = "DeleteKey")]
    DeleteKey,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "app_staff")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub application_id: Uuid,
    #[sea_orm(indexed)]
    pub user_id: Uuid,
    pub permissions: Vec<AppStaffPermissions>,
    #[sea_orm(default_value = "false")]
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
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

        if insert {
            s.id = ActiveValue::set(Uuid::new_v4());
        }
        s.updated_at = ActiveValue::set(OffsetDateTime::now_utc());
        Ok(s)
    }
}
