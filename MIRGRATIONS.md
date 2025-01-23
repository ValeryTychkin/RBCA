# Migrations

Unfortunately, SeaORM does not support automatic migration generation based on entities. 
This means you need to write the migration logic manually. 
While this adds extra work, it also gives you fine-grained control over the migration process.

## Steps to Write a Migration:

1. Create a new migration, use the following command:  
  ```bash
  cargo gen_migration {migration_name}
  ```
2. After running the `cargo gen_migration` command, find the new file in the migrations directory. File name: `{timestamp}_{migration_name}.rs`
3. Write a Migration
4. Test Your Migratio

---

## F.A.Q

1. **How do i know which Rust type converts to which DB type?**  
  It is in this table: https://www.sea-ql.org/SeaORM/docs/next/generate-entity/entity-structure/#column-type
2. **What's varchar len in default `String` type?**  
  By default 255
3. **How can i specify the length of `String`**
  ```rust
  pub struct Model {
    #[sea_orm(column_type = "String(StringLen::N(1024))")]
    pub title: String,
  }
  ```
