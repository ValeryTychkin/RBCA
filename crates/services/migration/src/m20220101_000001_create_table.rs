use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"CREATE TABLE IF NOT EXISTS "organization" (
                "id" UUID PRIMARY KEY,
                "display_name" VARCHAR(255) NOT NULL,
                "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                "is_deleted" BOOLEAN NOT NULL DEFAULT FALSE
            )"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE TABLE IF NOT EXISTS "user" (
                "id" UUID PRIMARY KEY,
                "name" VARCHAR(255) NOT NULL,
                "email" VARCHAR(255) UNIQUE NOT NULL,
                "birthday" DATE NOT NULL,
                "organization_id" UUID NOT NULL,
                "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                "is_deleted" BOOLEAN NOT NULL DEFAULT FALSE,
                CONSTRAINT "user_organization_fk" FOREIGN KEY ("organization_id") 
                    REFERENCES "organization" ("id") ON DELETE CASCADE ON UPDATE CASCADE
            )"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE INDEX IF NOT EXISTS "idx-user-organization_id" ON "user" ("organization_id")"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE UNIQUE INDEX IF NOT EXISTS "idx-user-email" ON "user" ("email")"#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(r#"DROP TABLE "organization""#)
            .await?;

        db.execute_unprepared(r#"DROP TABLE "user""#).await?;

        db.execute_unprepared(r#"DROP INDEX "idx-user-organization_id""#)
            .await?;

        db.execute_unprepared(r#"DROP INDEX "idx-user-email""#)
            .await?;

        Ok(())
    }
}
