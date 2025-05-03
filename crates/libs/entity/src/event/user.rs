use repository_amqp_lib::event::user::{self as event_user, EventType};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entities::user as user_entity;

/// Represents a user with necessary fields for business logic and events.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    name: String,
    is_deleted: bool,
    #[serde(skip)]
    active_model: user_entity::ActiveModel,
}

impl User {
    /// Creates a new `User` instance from an `ActiveModel` representation.
    ///
    /// # Arguments
    /// * `user` - The `ActiveModel` representation of a user entity.
    pub fn new_from_am(active_model: &user_entity::ActiveModel) -> Self {
        User {
            id: active_model.id.clone().take().unwrap(),
            name: active_model.name.clone().take().unwrap(),
            is_deleted: active_model.is_deleted.clone().take().unwrap_or(false),
            active_model: active_model.to_owned(),
        }
    }

    pub fn is_changed(&self) -> bool {
        !self.active_model.name.is_unchanged() || !self.active_model.is_deleted.is_unchanged()
    }

    pub fn is_deleted(&self) -> bool {
        !self.active_model.is_deleted.is_unchanged() && self.is_deleted == true
    }
}

/// Creates or updates a user in the system, publishing relevant events.
///
/// # Arguments
/// * `user` - The `ActiveModel` representation of a user entity.
///
/// Publishes:
/// - "Create" event if the user is new.
/// - "Update" event if the user has changed.
/// - "Delete" event if the user is marked as deleted.
pub async fn create_or_update(active_model: &user_entity::ActiveModel, insert: bool) {
    let user = User::new_from_am(&active_model);
    let user_json = serde_json::to_string(&user).unwrap();
    let mut event_type: Option<EventType> = None;
    if insert {
        event_type = Some(event_user::EventType::Create);
    } else {
        if user.is_changed() {
            if user.is_deleted() {
                event_type = Some(event_user::EventType::Delete);
            } else {
                event_type = Some(event_user::EventType::Delete);
            }
        }
    }
    if let Some(event_type) = event_type {
        event_user::publish(&user_json, event_type).await;
    }
}

/// Deletes a user from the system, publishing a "Delete" event.
///
/// # Arguments
/// * `user` - The `ActiveModel` representation of a user entity.
pub async fn delete(user: &user_entity::ActiveModel) {
    let user = User::new_from_am(user);
    event_user::publish(
        &serde_json::to_string(&user).unwrap(),
        event_user::EventType::Delete,
    )
    .await;
}
