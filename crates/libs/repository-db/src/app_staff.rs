use adapter_lib::db;
use async_trait::async_trait;
pub use entity_lib::app_staff as app_staff_entity;
use sea_orm::DatabaseConnection;

pub use crate::Repository;

pub struct AppStaff {
    db: &'static DatabaseConnection,
}

#[async_trait]
impl Repository<app_staff_entity::Entity> for AppStaff {
    async fn new() -> Self {
        Self {
            db: db::get_connection().await,
        }
    }

    async fn get_db(&self) -> &'static DatabaseConnection {
        self.db
    }
}
