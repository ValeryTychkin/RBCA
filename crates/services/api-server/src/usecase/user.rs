use crate::{query::user as user_query, schema::user as user_schema};
use orm_util_lib::prelude::EntityFilterableTrait;
use repository_db_lib::{
    user::{user_entity, User as UserRep},
    Repository,
};
use sea_orm::{ColumnTrait, Condition, Set};
use uuid::Uuid;

pub enum ErrorUpdate {
    UserNotFound,
}

pub enum ErrorCreate {
    EmailAllreadyExist,
}

pub async fn get_all(query_filter: &user_query::User) -> user_schema::UserList {
    // Get filter
    let filter = query_filter.to_condition::<user_entity::Entity>();
    // Get Models
    let rep = UserRep::new().await;
    let (user_models, limit, offset, total_count) = rep
        .get_multiple(
            Some(filter.to_owned()),
            query_filter.offset,
            query_filter.limit,
        )
        .await
        .unwrap();
    user_schema::UserList::from_models(&user_models, limit, offset, total_count)
}

pub async fn create(
    new_user: &user_schema::CreateUser,
    password: Option<&str>,
) -> Result<user_schema::User, ErrorCreate> {
    // Check if email allready exist
    let filter = Condition::all().add(user_entity::Column::Email.eq(new_user.email.to_owned()));
    let rep = UserRep::new().await;
    if let Some(_) = rep.get_one(Some(filter)).await.unwrap() {
        return Err(ErrorCreate::EmailAllreadyExist);
    }

    let password: String = match password {
        Some(v) => v.to_string(),
        None => user_entity::Model::gen_password(),
    };

    // Save new user
    let user_model = user_entity::ActiveModel {
        name: Set(new_user.name.to_owned()),
        email: Set(new_user.email.to_owned()),
        password: Set(password),
        birthday: Set(new_user.birthday),
        is_staff: Set(new_user.is_staff.unwrap_or(false)),
        ..Default::default()
    };
    // Convert Model into Schema
    let user_model = rep.create(user_model).await.unwrap();
    Ok(user_schema::User::from_model(&user_model))
}

pub async fn update(
    user_id: Uuid,
    is_staff: bool,
    user: &user_schema::UpdateUser,
) -> Result<user_schema::User, ErrorUpdate> {
    let rep = UserRep::new().await;
    let filter = Condition::all()
        .add(user_entity::Column::Id.eq(user_id))
        .add(user_entity::Column::IsStaff.eq(is_staff));

    // Try to get user by id
    let user_model: user_entity::Model = match rep.get_one(Some(filter)).await.unwrap() {
        Some(v) => v,
        None => return Err(ErrorUpdate::UserNotFound),
    };

    // Convert user model into active model
    let mut user_model: user_entity::ActiveModel = user_model.into();
    user_model.name = Set(user.name.to_owned());
    user_model.birthday = Set(user.birthday.to_owned());

    // Convert Model into Schema
    let user_model = rep.update(user_model).await.unwrap();
    Ok(user_schema::User::from_model(&user_model))
}

pub enum ErrorUpdatePassword {
    UserNotFound,
    WrongOldPassword,
}

pub async fn update_password(
    user_id: Uuid,
    passwords: &user_schema::UpdateUserPassword,
) -> Result<user_schema::User, ErrorUpdatePassword> {
    let rep = UserRep::new().await;
    let filter = Condition::all().add(user_entity::Column::Id.eq(user_id));

    // Try to get user by id
    let user_model: user_entity::Model = match rep.get_one(Some(filter)).await.unwrap() {
        Some(v) => v,
        None => return Err(ErrorUpdatePassword::UserNotFound),
    };

    if !user_model.is_valid_password(&passwords.old_password) {
        return Err(ErrorUpdatePassword::WrongOldPassword);
    }
    // Convert user model into active model
    let mut user_model: user_entity::ActiveModel = user_model.into();
    user_model.password = Set(passwords.new_password.to_owned());
    // Convert Model into Schema
    let user_model = rep.update(user_model).await.unwrap();
    Ok(user_schema::User::from_model(&user_model))
}
