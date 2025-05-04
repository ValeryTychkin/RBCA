use crate::{
    guard::user::{self as user_guard, User},
    query::application as application_query,
    schema::application as application_schema,
};
use orm_util_lib::prelude::EntityFilterableTrait;
use repository_db_lib::{
    app_staff::{app_staff_entity, AppStaff as AppStaffRep},
    application::{application_entity, Application as ApplicationRep},
    Repository,
};
use sea_orm::{ColumnTrait, Condition, Set};
use uuid::Uuid;

pub enum ErrorCreate {
    ApplicationNameAllreadyExist,
    AddCreatorIntoNewApplication,
}

pub async fn create(
    creator: &user_guard::User,
    new_application: &application_schema::CreateApplication,
) -> Result<application_schema::Application, ErrorCreate> {
    let rep = ApplicationRep::new().await;

    // Check if application by name allready exist
    let filter =
        Condition::all().add(application_entity::Column::Name.eq(new_application.name.to_owned()));
    if let Some(_) = rep.get_one(Some(filter)).await.unwrap() {
        return Err(ErrorCreate::ApplicationNameAllreadyExist);
    }

    // Save new application
    let application_model = application_entity::ActiveModel {
        name: Set(new_application.name.to_owned()),
        description: Set(new_application.description.to_owned()),
        ..Default::default()
    };
    let application_model = rep.create(application_model).await.unwrap();

    if add_staff(
        application_model.id.to_owned(),
        creator.claims.id.to_owned(),
        app_staff_entity::AppStaffPermissions::get_all(),
    )
    .await
    .is_err()
    {
        let _ = rep.delete_by_id(application_model.id).await;
        return Err(ErrorCreate::AddCreatorIntoNewApplication);
    }

    Ok(application_schema::Application::from_model(
        &application_model,
    ))
}

pub async fn get_all(
    user: User,
    query_filter: &application_query::Application,
) -> application_schema::ApplicationList {
    // Get filter
    let filter = query_filter
        .to_condition::<application_entity::Entity>()
        .add(app_staff_entity::Column::UserId.eq(user.claims.id))
        .add(
            app_staff_entity::Column::Permissions
                .contains(app_staff_entity::AppStaffPermissions::ReadApplication.to_string()),
        );
    let rep = ApplicationRep::new().await;

    // Get accessed
    let (app_models, limit, offset, total_count) = rep
        .get_multiple_with_app_staff(Some(filter), query_filter.offset, query_filter.limit)
        .await
        .unwrap();

    application_schema::ApplicationList::from_models(&app_models, limit, offset, total_count)
}

pub enum ErrorAddStaff {
    ErrorCreate,
}

async fn add_staff(
    application_id: Uuid,
    user_id: Uuid,
    permissions: Vec<app_staff_entity::AppStaffPermissions>,
) -> Result<application_schema::ApplicationStaff, ErrorAddStaff> {
    let app_staff_rep = AppStaffRep::new().await;
    let app_staff_model = app_staff_entity::ActiveModel {
        application_id: Set(application_id),
        user_id: Set(user_id),
        permissions: Set(permissions),
        ..Default::default()
    };

    let app_staff_model = match app_staff_rep.create(app_staff_model).await {
        Ok(v) => v,
        Err(_) => return Err(ErrorAddStaff::ErrorCreate),
    };

    Ok(application_schema::ApplicationStaff::from_model(
        &app_staff_model,
    ))
}
