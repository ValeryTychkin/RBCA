use sea_orm::{
    entity::{prelude::*, ActiveValue},
    QuerySelect,
};
use util_lib::crypto::{Bcrypt, Hasher};
use uuid::Uuid;

use crate::event::user as user_event;

use time::{Date, OffsetDateTime};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    #[sea_orm(unique, indexed)]
    pub email: String,
    pub password: String,
    pub birthday: Date,
    #[sea_orm(indexed)]
    pub organization_id: Uuid,
    #[sea_orm(default_value = "false")]
    pub is_deleted: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Model {
    pub fn is_valid_password(&self, password: &str) -> bool {
        Bcrypt::new().verify(password, self.password.as_str())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrganizationId",
        to = "super::organization::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Organization,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut s = self;

        let bcrypt = Bcrypt::new();
        if insert {
            s.id = ActiveValue::set(Uuid::new_v4());
            s.password = ActiveValue::set(bcrypt.hash(s.password.unwrap().as_str()));
        } else {
            let old_password: String = Entity::find_by_id(s.id.clone().unwrap())
                .column(Column::Password)
                .into_tuple()
                .one(db)
                .await
                .unwrap()
                .unwrap();

            // Check password on update (save hash if update)
            if s.password.clone().unwrap() != old_password {
                s.password = ActiveValue::set(bcrypt.hash(s.password.unwrap().as_str()));
            }
        }
        s.updated_at = ActiveValue::set(OffsetDateTime::now_utc());
        user_event::create_or_update(&s, db).await;
        Ok(s)
    }

    async fn before_delete<C>(self, db: &C) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        user_event::delete(&self, db).await;
        Ok(self)
    }
}
