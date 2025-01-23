use repository_amqp_lib::event::user as event_user;
use sea_orm::{ConnectionTrait, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::user as user_entity;

/// Represents a user with necessary fields for business logic and events.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub is_deleted: bool,
}

impl User {
    /// Creates a new `User` instance from an `ActiveModel` representation.
    ///
    /// # Arguments
    /// * `user` - The `ActiveModel` representation of a user entity.
    pub fn new_from_am(user: &user_entity::ActiveModel) -> Self {
        User {
            id: user.id.clone().take().unwrap(),
            name: user.name.clone().take().unwrap(),
            is_deleted: user.is_deleted.clone().take().unwrap_or(false),
        }
    }

    /// Checks if the current `User` instance differs from the provided database model.
    pub fn is_changed(&self, user_model: &user_entity::Model) -> bool {
        if self.name != user_model.name || self.is_deleted {
            return true;
        }
        return false;
    }
}

/// Creates or updates a user in the system, publishing relevant events.
///
/// # Arguments
/// * `user` - The `ActiveModel` representation of a user entity.
/// * `db` - The database connection to use.
///
/// Publishes:
/// - A "Create" event if the user is new.
/// - An "Update" event if the user has changed.
/// - A "Delete" event if the user is marked as deleted.
pub async fn create_or_update<C>(user: &user_entity::ActiveModel, db: &C)
where
    C: ConnectionTrait,
{
    let user = User::new_from_am(user);
    if let Some(user_model) = user_entity::Entity::find_by_id(user.id)
        .one(db)
        .await
        .unwrap()
    {
        if user.is_changed(&user_model) {
            if user.is_deleted {
                event_user::publish(
                    &serde_json::to_string(&user).unwrap(),
                    event_user::EventType::Delete,
                )
                .await;
                return;
            }
            event_user::publish(
                &serde_json::to_string(&user).unwrap(),
                event_user::EventType::Update,
            )
            .await;
            return;
        }
    }
    event_user::publish(
        &serde_json::to_string(&user).unwrap(),
        event_user::EventType::Create,
    )
    .await;
    return;
}

/// Deletes a user from the system, publishing a "Delete" event.
///
/// # Arguments
/// * `user` - The `ActiveModel` representation of a user entity.
/// * `_db` - The database connection (unused in this function).
pub async fn delete<C>(user: &user_entity::ActiveModel, _db: &C)
where
    C: ConnectionTrait,
{
    let user = User::new_from_am(user);
    event_user::publish(
        &serde_json::to_string(&user).unwrap(),
        event_user::EventType::Delete,
    )
    .await;
}
