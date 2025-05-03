use adapter_lib::db;
use async_trait::async_trait;
pub use entity_lib::user as user_entity;
use sea_orm::{prelude::Expr, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::builder::QueryBuilder;
pub use crate::Repository;

pub struct User {
    db: &'static DatabaseConnection,
}

impl QueryBuilder<user_entity::Entity> for User {}

#[async_trait]
impl Repository<user_entity::Entity> for User {
    async fn new() -> Self {
        Self {
            db: db::get_connection().await,
        }
    }

    async fn get_db(&self) -> &'static DatabaseConnection {
        self.db
    }

    async fn delete(&self, filter: Condition) -> Result<(), DbErr> {
        let db = self.get_db().await;
        match user_entity::Entity::update_many()
            .col_expr(user_entity::Column::IsDeleted, Expr::value(true))
            .filter(filter)
            .exec(db)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
