use crate::{
    query::user as user_query,
    schema::{base::Pagination, user as user_schema},
};
use orm_util_lib::prelude::EntityFilterableTrait;
use repository_db_lib::{
    user::{user_entity, User as UserRep},
    Repository,
};

pub async fn get_all(query_filter: user_query::User) -> user_schema::UserList {
    // Get filter
    let filter = query_filter.to_condition::<user_entity::Entity>();
    // Get Models
    let rep = UserRep::new().await;
    let (models, offset, limit, total_count) = rep
        .get_multiple(
            Some(filter.to_owned()),
            query_filter.offset,
            query_filter.limit,
            true,
        )
        .await
        .unwrap();
    // Convert Models into Schemes
    let mut users: Vec<user_schema::User> = vec![];
    for model in models {
        users.push(user_schema::User {
            id: model.id,
            name: model.name,
            email: model.email,
            is_deleted: model.is_deleted,
            created_at: model.created_at,
            updated_ad: model.updated_at,
        });
    }
    user_schema::UserList {
        users,
        pagination: Pagination {
            limit,
            offset,
            total: total_count,
        },
    }
}
