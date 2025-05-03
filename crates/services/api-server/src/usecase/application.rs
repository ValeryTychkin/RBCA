use crate::{
    guard::user::{self as user_guard, User},
    query::application as application_query,
    schema::{application as application_schema, Pagination},
};
use orm_util_lib::prelude::EntityFilterableTrait;
use repository_db_lib::{
    app_staff::{app_staff_entity, AppStaff as AppStaffRep},
    application::{application_entity, Application as ApplicationRep},
    Repository,
};
use sea_orm::{ColumnTrait, Condition, Set};

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

    // TODO Move to app_staff usecase
    let app_staff_rep = AppStaffRep::new().await;
    let app_staff_model = app_staff_entity::ActiveModel {
        application_id: Set(application_model.id.to_owned()),
        user_id: Set(creator.claims.id.to_owned()),
        permissions: Set(app_staff_entity::AppStaffPermissions::get_all()),
        ..Default::default()
    };

    match app_staff_rep.create(app_staff_model).await {
        Ok(v) => v,
        Err(_) => {
            rep.delete_by_id(application_model.id).await;
            return Err(ErrorCreate::AddCreatorIntoNewApplication);
        }
    };

    // TODO new application
    Ok(model_into_schema(&application_model))
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
    // Convert Models into Schemes
    let mut applications = Vec::<application_schema::Application>::new();
    for app_model in app_models.iter() {
        applications.push(model_into_schema(app_model));
    }
    application_schema::ApplicationList {
        applications,
        pagination: Pagination {
            limit,
            offset,
            total: total_count,
        },
    }
}

pub fn model_into_schema(model: &application_entity::Model) -> application_schema::Application {
    application_schema::Application {
        id: model.id.to_owned(),
        name: model.name.to_owned(),
        description: model.description.to_owned(),
        updated_at: model.updated_at.to_owned(),
        created_at: model.created_at.to_owned(),
    }
}
