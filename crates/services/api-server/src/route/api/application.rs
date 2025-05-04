use crate::{
    guard::{staff::user::UserStaff as GuardUserStaff, GuardError},
    merdge_mulit_routes,
    query::application as application_query,
    schema::{self, application as application_schema, user as user_schema},
    usecase::application as application_usecase,
};
use rocket::{http::Status, serde::json::Json};
use rocket_okapi::{
    get_nested_endpoints_and_docs, okapi::openapi3::OpenApi, openapi, settings::OpenApiSettings,
};
use rocket_util_lib::guard_permission;

#[openapi(tag = "Application")]
#[guard_permission(error_ty = GuardError, perm_error = MissingPermission, all_perms = (user_schema::StaffPermission::CreateApplication))]
#[post("/", data = "<new_application>")]
pub async fn create(
    guard: GuardUserStaff,
    new_application: Json<application_schema::CreateApplication>,
) -> (
    Status,
    Result<Json<application_schema::Application>, Json<schema::ErrorResult>>,
) {
    match application_usecase::create(&guard.user, &new_application.0).await {
        Ok(v) => (Status::Created, Ok(Json(v))),
        Err(e) => match e {
            application_usecase::ErrorCreate::ApplicationNameAllreadyExist => (
                Status::Conflict,
                Err(Json(schema::ErrorResult {
                    err_type: schema::ErrorType::Conflict,
                    err_msg: "application name allready exist".to_string(),
                    err_detail: None,
                })),
            ),
            application_usecase::ErrorCreate::AddCreatorIntoNewApplication => (
                Status::InternalServerError,
                Err(Json(schema::ErrorResult {
                    err_type: schema::ErrorType::Conflict,
                    err_msg: "error to create application".to_string(),
                    err_detail: None,
                })),
            ),
        },
    }
}

#[openapi(tag = "Application")]
#[get("/?<req_query..>")]
pub async fn get_multiple(
    guard: GuardUserStaff,
    req_query: application_query::Application,
) -> (Status, Json<application_schema::ApplicationList>) {
    (
        Status::Ok,
        Json(application_usecase::get_all(guard.user, &req_query).await),
    )
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    get_nested_endpoints_and_docs! {
        "/" => merdge_mulit_routes![settings, [create, get_multiple]],
    }
}
