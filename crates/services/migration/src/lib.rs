use adapter_lib::db;
use sea_orm::DatabaseConnection;
pub use sea_orm_migration::prelude::*;

mod m20250503_000001_create_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250503_000001_create_tables::Migration)]
    }
}

pub async fn init() {
    let db: &DatabaseConnection = db::get_connection().await;
    Migrator::up(db, None).await.unwrap();
}
