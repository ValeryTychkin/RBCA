use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(r#"ALTER TABLE "user" DROP COLUMN "organization_id""#)
            .await?;

        db.execute_unprepared(r#"DROP TABLE IF EXISTS "organization""#)
            .await?;

        db.execute_unprepared(
            r#"ALTER TABLE "user" ADD COLUMN "is_staff" BOOLEAN NOT NULL DEFAULT FALSE"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE TYPE "user_staff_permission" AS ENUM 
                ('CreateApplication', 'CreateStaffUser', 'DeleteStaffUser', 'UpdateStaffUser', 'DeleteUser')"#,
        )
        .await?;

        db.execute_unprepared(
            r#"ALTER TABLE "user" ADD COLUMN "staff_permissions" USER_STAFF_PERMISSION[] NOT NULL DEFAULT ARRAY[]::USER_STAFF_PERMISSION[]"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE TABLE IF NOT EXISTS "application" (
                "id" UUID PRIMARY KEY,
                "name" VARCHAR(255) NOT NULL,
                "description" VARCHAR(2048) NOT NULL DEFAULT '',
                "is_deleted" BOOLEAN NOT NULL DEFAULT FALSE,
                "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            )"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE INDEX IF NOT EXISTS "idx-application-name" ON "application" ("name")"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE TYPE "app_staff_permission" AS ENUM
                ('UpdateApplication', 'ReadApplication', 'DeleteApplication', 'CreateKey', 'ReadKey', 'ReadKeyDetail', 'UpdateKey', 'DeleteKey')"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE TABLE IF NOT EXISTS "app_staff" (
                "id" UUID PRIMARY KEY,
                "application_id" UUID NOT NULL,
                "user_id" UUID NOT NULL,
                "permissions" APP_STAFF_PERMISSION[] NOT NULL DEFAULT ARRAY[]::APP_STAFF_PERMISSION[],
                "is_deleted" BOOLEAN NOT NULL DEFAULT FALSE,
                "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                CONSTRAINT "app_staff_application_fk" FOREIGN KEY ("application_id")
                    REFERENCES "application" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
                CONSTRAINT "app_staff_user_fk" FOREIGN KEY ("user_id")
                    REFERENCES "user" ("id") ON DELETE CASCADE ON UPDATE CASCADE
            )"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE INDEX IF NOT EXISTS "idx-app_staff-application_id" ON "app_staff" ("application_id")"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE INDEX IF NOT EXISTS "idx-app_staff-user_id" ON "app_staff" ("user_id")"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE TABLE IF NOT EXISTS "key" (
                "id" UUID PRIMARY KEY,
                "value" VARCHAR(255) NOT NULL,
                "activated_at" TIMESTAMPTZ,
                "lifetime" BIGINT NOT NULL,
                "is_bunned" BOOLEAN NOT NULL DEFAULT FALSE,
                "application_id" UUID NOT NULL,
                "user_id" UUID NOT NULL,
                "created_by_user_id" UUID NOT NULL,
                "is_deleted" BOOLEAN NOT NULL DEFAULT FALSE,
                "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                CONSTRAINT "key_application_fk" FOREIGN KEY ("application_id")
                    REFERENCES "application" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
                CONSTRAINT "key_user_fk" FOREIGN KEY ("user_id")
                    REFERENCES "user" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
                CONSTRAINT "key_created_by_user_fk" FOREIGN KEY ("created_by_user_id")
                    REFERENCES "user" ("id") ON DELETE CASCADE ON UPDATE CASCADE
            )"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE INDEX IF NOT EXISTS "idx-key-application_id" ON "key" ("application_id")"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE INDEX IF NOT EXISTS "idx-key-user_id" ON "key" ("user_id")"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE INDEX IF NOT EXISTS "idx-key-created_by_user_id" ON "key" ("created_by_user_id")"#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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

        db.execute_unprepared(r#"ALTER TABLE "user" ADD COLUMN "organization_id" UUID NOT NULL"#)
            .await?;

        db.execute_unprepared(
            r#"ALTER TABLE "user" ADD CONSTRAINT "user_organization_fk" FOREIGN KEY ("organization_id") 
                REFERENCES "organization" ("id") ON DELETE CASCADE ON UPDATE CASCADE"#
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE INDEX IF NOT EXISTS "idx-user-organization_id" ON "user" ("organization_id")"#,
        )
        .await?;

        db.execute_unprepared(r#"ALTER TABLE "user" DROP COLUMN "staff_permissions""#)
            .await?;

        db.execute_unprepared(r#"DROP TYPE IF EXISTS "user_staff_permission""#)
            .await?;

        db.execute_unprepared(r#"ALTER TABLE "user" DROP COLUMN "is_staff""#)
            .await?;

        db.execute_unprepared(r#"DROP TABLE IF EXISTS "application""#)
            .await?;

        db.execute_unprepared(r#"DROP TYPE IF EXISTS "app_staff_permission""#)
            .await?;

        db.execute_unprepared(r#"DROP TABLE IF EXISTS "app_staff""#)
            .await?;

        db.execute_unprepared(r#"DROP TABLE IF EXISTS "key""#)
            .await?;

        Ok(())
    }
}
