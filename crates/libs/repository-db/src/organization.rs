use adapter_lib::db;
use async_trait::async_trait;
pub use entity_lib::organization as org_entity;
use sea_orm::DatabaseConnection;

pub use crate::Repository;

pub struct Organization {
    db: &'static DatabaseConnection,
}

#[async_trait()]
impl Repository<org_entity::Entity> for Organization {
    async fn new() -> Self {
        Organization {
            db: db::get_connection().await,
        }
    }

    async fn get_db(&self) -> &'static DatabaseConnection {
        self.db
    }
}
