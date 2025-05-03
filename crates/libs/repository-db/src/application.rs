use adapter_lib::db;
use async_trait::async_trait;
pub use entity_lib::{app_staff as app_staff_entity, application as application_entity};
use orm_util_lib::{get_limit, get_offset};
use sea_orm::{
    prelude::Expr, Condition, DatabaseConnection, DbErr, EntityTrait, JoinType, PaginatorTrait,
    QueryFilter, QuerySelect, RelationTrait,
};

use crate::builder::QueryBuilder;
pub use crate::Repository;

pub struct Application {
    db: &'static DatabaseConnection,
}

impl Application {
    pub async fn get_multiple_with_app_staff(
        &self,
        filter: Option<Condition>,
        offset: Option<u64>,
        limit: Option<i64>,
    ) -> Result<(Vec<application_entity::Model>, i64, u64, u64), DbErr> {
        let limit = get_limit(limit);
        let offset = get_offset(offset);

        let db = self.get_db().await;

        let models = match Self::select(filter.to_owned(), limit, offset)
            .join(
                JoinType::InnerJoin,
                app_staff_entity::Relation::Application.def(),
            )
            .all(db)
            .await
        {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let total_count = match Self::select(filter, -1, 0)
            .join(
                JoinType::InnerJoin,
                app_staff_entity::Relation::Application.def(),
            )
            .count(db)
            .await
        {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        Ok((models, limit, offset, total_count))
    }
}

impl QueryBuilder<application_entity::Entity> for Application {}

#[async_trait]
impl Repository<application_entity::Entity> for Application {
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
        match application_entity::Entity::update_many()
            .col_expr(application_entity::Column::IsDeleted, Expr::value(true))
            .filter(filter)
            .exec(db)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
