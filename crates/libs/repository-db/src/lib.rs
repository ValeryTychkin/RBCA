pub mod user;

use std::u64;

use async_trait::async_trait;
use orm_util_lib::{LIMIT_DEFAULT, LIMIT_MAX, LIMIT_MIN, OFFSET_DEFAULT};
use sea_orm::{
    ActiveModelTrait, Condition, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    PaginatorTrait, PrimaryKeyTrait, QueryFilter, QuerySelect,
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
        limit: Option<u64>,
        with_total_count: bool,
    ) -> Result<(Vec<E::Model>, u64, u64, u64), DbErr> {
        // Check optional offset on max/min value or None (if None set default)
        let opt_offset = offset;
        let offset: u64;
        match opt_offset {
            Some(v) => {
                offset = v;
            }
            None => {
                offset = OFFSET_DEFAULT;
            }
        }

        // Check optional limit on max/min value or None (if None set default)
        let opt_limit = limit;
        let limit: u64;
        match opt_limit {
            Some(v) => {
                if v < LIMIT_MIN {
                    limit = LIMIT_MIN;
                } else if v > LIMIT_MAX {
                    limit = LIMIT_MAX;
                } else {
                    limit = v;
                }
            }
            None => {
                limit = LIMIT_DEFAULT;
            }
        }

        // Try to unwrap filter or set withoute filter (all)
        let filter: Condition = filter.unwrap_or(Condition::all());

        let db = self.get_db().await;

        let models_res = E::find()
            .filter(filter.to_owned())
            .offset(offset)
            .limit(limit)
            .all(db)
            .await;

        match models_res {
            Ok(models) => {
                let mut total_count = 0;
                if with_total_count {
                    let total_count_res = self.get_total_count(Some(filter)).await;
                    match total_count_res {
                        Ok(v) => {
                            total_count = v;
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                Ok((models, offset, limit, total_count))
            }
            Err(e) => Err(e),
        }
    }

    async fn get_total_count(&self, filter: Option<Condition>) -> Result<u64, DbErr> {
        let filter: Condition = filter.unwrap_or(Condition::all());
        let db = self.get_db().await;
        E::find().filter(filter).count(db).await
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

    async fn get_by_id<Pk>(&self, id: Pk) -> Result<Option<E::Model>, DbErr>
    where
        Pk: Into<<E::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync + Clone,
    {
        let db = self.get_db().await;
        E::find_by_id(id).one(db).await
    }
}
