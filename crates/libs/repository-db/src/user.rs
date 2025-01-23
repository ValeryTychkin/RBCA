use adapter_lib::db;
use async_trait::async_trait;
pub use entity_lib::user as user_entity;
use sea_orm::DatabaseConnection;

pub use crate::Repository;

pub struct User {
    db: &'static DatabaseConnection,
}

#[async_trait]
impl Repository<user_entity::Entity> for User {
    async fn new() -> Self {
        User {
            db: db::get_connection().await,
        }
    }

    async fn get_db(&self) -> &'static DatabaseConnection {
        self.db
    }
}
