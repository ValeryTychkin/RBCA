use sea_orm::entity::{prelude::*, ActiveValue};
use uuid::Uuid;

use time::OffsetDateTime;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "key")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique, indexed)]
    pub value: String,
    pub activated_at: Option<OffsetDateTime>,
    pub lifetime: u32,
    #[sea_orm(default_value = "false")]
    pub is_bunned: bool,
    #[sea_orm(indexed)]
    pub application_id: Uuid,
    #[sea_orm(indexed)]
    pub user_id: Uuid,
    #[sea_orm(indexed)]
    pub created_by_user_id: Uuid,
    #[sea_orm(default_value = "false")]
    pub is_deleted: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Model {
    pub fn is_not_expired(&self) -> bool {
        match self.activated_at {
            Some(v) => {
                (v.unix_timestamp()
                    - OffsetDateTime::now_utc()
                        .to_offset(v.offset())
                        .unix_timestamp())
                    < self.lifetime as i64
            }
            None => true,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::CreatedByUserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    CreatedByUser,
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
