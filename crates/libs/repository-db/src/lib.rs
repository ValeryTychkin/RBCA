pub mod app_staff;
pub mod application;
pub mod user;

use async_trait::async_trait;
use orm_util_lib::{get_limit, get_offset};
use sea_orm::{
    sea_query::IntoValueTuple, ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr,
    EntityTrait, IntoActiveModel, Iterable, PaginatorTrait, PrimaryKeyToColumn, PrimaryKeyTrait,
    QueryFilter, QuerySelect,
};

/// A trait that defines common repository methods for working with entities.
///
/// The `Repository` trait provides basic CRUD operations and utility methods for entities that
/// implement the `EntityTrait` from SeaORM. The trait is generic and works with any entity type `E` that
/// satisfies the required bounds.
#[async_trait]
pub trait Repository<E>: Send + Sync
where
    E: EntityTrait + Send + Sync,
    E::PrimaryKey: PrimaryKeyTrait,
    E::Model: Send + Sync + IntoActiveModel<E::ActiveModel>,
    E::ActiveModel: ActiveModelTrait<Entity = E> + Send + Sync + From<E::Model>,
{
    /// Initializes a new repository instance.
    ///
    /// # Returns
    /// A new repository instance.
    ///
    /// # Example
    /// ```rust
    /// let repo = YourRepository::new().await;
    /// ```
    async fn new() -> Self;

    async fn get_db(&self) -> &'static DatabaseConnection;

    /// Retrieves multiple `Self::Model` records from the database.
    ///
    /// This function accepts a vector of `Condition` objects, combines them
    /// into a single filter condition, and queries the database for matching records.
    /// It returns a vector of results, unwrapping the operation for simplicity.
    ///
    /// # Parameters
    /// - `filters`: A vector of `Condition` objects to filter the query results.
    ///
    /// # Returns
    /// A vector of `Self::Model` instances matching the combined conditions.
    async fn get_multiple(
        &self,
        filter: Option<Condition>,
        offset: Option<u64>,
        limit: Option<i64>,
    ) -> Result<(Vec<E::Model>, i64, u64, u64), DbErr>
    where
        Self: builder::QueryBuilder<E>,
    {
        let limit = get_limit(limit);
        let offset = get_offset(offset);

        let db = self.get_db().await;

        let models = match Self::select(filter.to_owned(), limit, offset).all(db).await {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        let total_count = match Self::select(filter, -1, 0).count(db).await {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        Ok((models, limit, offset, total_count))
    }

    async fn is_exist(&self, filter: Option<Condition>) -> Result<bool, DbErr> {
        let filter: Condition = filter.unwrap_or(Condition::all());
        let db = self.get_db().await;
        match E::find().filter(filter).limit(1).count(db).await {
            Ok(count) => Ok(count > 0),
            Err(e) => Err(e),
        }
    }

    async fn create(&self, active_model: E::ActiveModel) -> Result<E::Model, DbErr> {
        let db = self.get_db().await;
        active_model.insert(db).await
    }

    async fn get_one(&self, filter: Option<Condition>) -> Result<Option<E::Model>, DbErr> {
        let filter: Condition = filter.unwrap_or(Condition::all());

        let db = self.get_db().await;
        E::find().filter(filter).one(db).await
    }

    async fn get_by_id<Pk>(&self, ids: Pk) -> Result<Option<E::Model>, DbErr>
    where
        Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync + Clone,
    {
        let db = self.get_db().await;
        E::find_by_id(ids).one(db).await
    }

    async fn update(&self, active_model: E::ActiveModel) -> Result<E::Model, DbErr> {
        let db = self.get_db().await;
        active_model.update(db).await
    }

    async fn delete_by_id<Pk>(&self, ids: Pk) -> Result<(), DbErr>
    where
        Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync + Clone,
    {
        let mut filter: Condition = Condition::all();

        // Collect all primary keys into a Vec before awaiting to ensure they are Send.
        // This prevents issues where the iterator itself is not Send and might be moved across threads
        // when the async function is suspended at an await point.
        let keys: Vec<_> = E::PrimaryKey::iter().collect();
        let mut keys = keys.into_iter();

        // Add filter by ids
        for id in ids.into().into_value_tuple() {
            if let Some(key) = keys.next() {
                let col = key.into_column();
                filter = filter.add(col.eq(id));
            } else {
                panic!("primary key arity mismatch");
            }
        }
        if keys.next().is_some() {
            panic!("primary key arity mismatch");
        }
        self.delete(filter).await
    }

    async fn delete(&self, filter: Condition) -> Result<(), DbErr> {
        let db = self.get_db().await;
        match E::delete_many().filter(filter).exec(db).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

mod builder {
    use orm_util_lib::{get_limit, get_offset};
    use sea_orm::{
        sea_query::IntoValueTuple, ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection,
        DbBackend, DbErr, EntityTrait, IntoActiveModel, Iterable, PaginatorTrait,
        PrimaryKeyToColumn, PrimaryKeyTrait, QueryFilter, QuerySelect, QueryTrait, Select,
    };

    pub(crate) trait QueryBuilder<E>
    where
        E: EntityTrait,
    {
        fn select(filter: Option<Condition>, limit: i64, offset: u64) -> Select<E> {
            // Try to unwrap filter or set withoute filter (all)
            let filter: Condition = filter.unwrap_or(Condition::all());

            let mut result = E::find().filter(filter.to_owned()).offset(offset);
            if limit >= 0 {
                result = result.limit(limit as u64);
            }
            result
        }
    }
}
