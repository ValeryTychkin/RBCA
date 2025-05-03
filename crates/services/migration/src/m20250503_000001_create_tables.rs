use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            r#"CREATE TABLE IF NOT EXISTS "application" (
                "id" UUID PRIMARY KEY,
                "name" VARCHAR(255) UNIQUE NOT NULL,
                "description" VARCHAR(2048) NOT NULL DEFAULT '',
                "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                "is_deleted" BOOLEAN NOT NULL DEFAULT FALSE
            )"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE INDEX "idx_application_name" ON "application" ("name");
            CREATE INDEX "idx_application_is_deleted" ON "application" ("is_deleted");"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE TABLE IF NOT EXISTS "user" (
                "id" UUID PRIMARY KEY,
                "name" VARCHAR(255) NOT NULL,
                "email" VARCHAR(255) NOT NULL,
                "password" VARCHAR(255) NOT NULL,
                "birthday" DATE NOT NULL,
                "is_staff" BOOLEAN NOT NULL DEFAULT FALSE,
                "staff_permissions" VARCHAR(255)[] NOT NULL DEFAULT '{}',
                "is_deleted" BOOLEAN NOT NULL DEFAULT FALSE,
                "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            )"#,
        )
        .await?;

        db.execute_unprepared(r#"CREATE INDEX "idx_user_is_deleted" ON "user" ("is_deleted");"#)
            .await?;

        db.execute_unprepared(
            r#"CREATE TABLE IF NOT EXISTS "app_staff" (
                "id" UUID PRIMARY KEY,
                "application_id" UUID NOT NULL,
                "user_id" UUID NOT NULL,
                "staff_permissions" VARCHAR(255)[] NOT NULL DEFAULT '{}',
                "is_deleted" BOOLEAN NOT NULL DEFAULT FALSE,
                "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                CONSTRAINT "fk_application_id" FOREIGN KEY ("application_id") REFERENCES "application" ("id") ON DELETE CASCADE,
                CONSTRAINT "fk_user_id" FOREIGN KEY ("user_id") REFERENCES "user" ("id") ON DELETE CASCADE
            )"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE INDEX "idx_app_staff_application_id" ON "app_staff" ("application_id");
            CREATE INDEX "idx_app_staff_user_id" ON "app_staff" ("user_id");
            CREATE UNIQUE INDEX "idx_app_staff_unique" ON "app_staff" ("application_id", "user_id");
            CREATE INDEX "idx_app_staff_is_deleted" ON "app_staff" ("is_deleted");"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE TABLE "key" (
                "id" UUID PRIMARY KEY,
                "value" VARCHAR NOT NULL UNIQUE,
                "activated_at" TIMESTAMPTZ,
                "lifetime" INTEGER NOT NULL CHECK (lifetime >= 0),
                "is_bunned" BOOLEAN NOT NULL,
                "application_id" UUID NOT NULL,
                "user_id" UUID NOT NULL,
                "created_by_user_id" UUID NOT NULL,
                "is_deleted" BOOLEAN NOT NULL DEFAULT FALSE,
                "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                CONSTRAINT "fk_application_id" FOREIGN KEY ("application_id") REFERENCES "application" ("id") ON DELETE CASCADE,
                CONSTRAINT "fk_user_id" FOREIGN KEY ("user_id") REFERENCES "user" ("id") ON DELETE CASCADE,
                CONSTRAINT "fk_created_by_user_id" FOREIGN KEY ("created_by_user_id") REFERENCES "user" ("id") ON DELETE CASCADE
            );"#,
        )
        .await?;

        db.execute_unprepared(
            r#"CREATE INDEX "idx_key_value" ON "key" ("value");
            CREATE INDEX "idx_key_application_id" ON "key" ("application_id");
            CREATE INDEX "idx_key_user_id" ON "key" ("user_id");
            CREATE INDEX "idx_key_created_by_user_id" ON "key" ("created_by_user_id");
            CREATE INDEX "idx_key_is_deleted" ON "key" ("is_deleted");"#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(r#"DROP TABLE "key"#).await?;

        db.execute_unprepared(r#"DROP TABLE "app_staff"#).await?;

        db.execute_unprepared(r#"DROP TABLE "application"#).await?;

        db.execute_unprepared(r#"DROP TABLE "user"#).await?;

        Ok(())
    }
}
