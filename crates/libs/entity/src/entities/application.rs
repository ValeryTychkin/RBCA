use sea_orm::entity::{prelude::*, ActiveValue};
use uuid::Uuid;

use time::OffsetDateTime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "application")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique, indexed)]
    pub name: String,
    #[sea_orm(column_type = "String(StringLen::N(2048))", default_value = "")]
    pub description: String,
    #[sea_orm(default_value = "false")]
    pub is_deleted: bool,
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
