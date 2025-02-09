use adapter_lib::db;
use sea_orm::DatabaseConnection;
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250110_205852_add_user_password;
mod m20250207_160853_del_org_add_app_key_upd_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250110_205852_add_user_password::Migration),
            Box::new(m20250207_160853_del_org_add_app_key_upd_user::Migration),
        ]
    }
}

pub async fn init() {
    let db: &DatabaseConnection = db::get_connection().await;
    Migrator::up(db, None).await.unwrap();
}
