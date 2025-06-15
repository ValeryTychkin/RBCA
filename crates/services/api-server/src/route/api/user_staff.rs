use crate::{
    guard::{staff::user::UserStaff as GuardUserStaff, GuardError},
    merdge_mulit_routes,
    query::user as user_query,
    schema::{self, user as user_schema},
    usecase::user as user_usecase,
};
use rocket::{http::Status, serde::json::Json};
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, settings::OpenApiSettings};
use rocket_util_lib::guard_permission;
use uuid::Uuid;

#[openapi(tag = "User Staff")]
#[guard_permission(error_ty = GuardError, perm_error = MissingPermission)]
#[get("/?<req_query..>")]
pub async fn get_multiple(
    _guard: GuardUserStaff,
    req_query: user_query::User,
) -> Json<user_schema::UserList> {
    let mut req_query = req_query;
    req_query.is_staff = Some(true);
    Json(user_usecase::get_all(&req_query).await)
}

#[openapi(tag = "User Staff")]
#[guard_permission(error_ty = GuardError, perm_error = MissingPermission, all_perms = [user_schema::StaffPermission::CreateStaffUser])]
#[post("/", data = "<new_user>")]
pub async fn create(
    _guard: GuardUserStaff,
    new_user: Json<user_schema::CreateUser>,
) -> (
    Status,
    Result<Json<user_schema::User>, Json<schema::ErrorResult>>,
) {
    let mut new_user = new_user.0;
    new_user.is_staff = Some(true);
    match user_usecase::create(&new_user, None).await {
        Ok(v) => (Status::Created, Ok(Json(v))),
        Err(e) => match e {
            user_usecase::ErrorCreate::EmailAllreadyExist => (
                Status::Conflict,
                Err(Json(schema::ErrorResult {
                    err_type: schema::ErrorType::Conflict,
                    err_msg: "email allready exist".to_string(),
                    err_detail: None,
                })),
            ),
        },
    }
}

#[openapi(tag = "User Staff")]
#[guard_permission(error_ty = GuardError, perm_error = MissingPermission, all_perms = [user_schema::StaffPermission::UpdateStaffUser])]
#[put("/", data = "<user>")]
pub async fn self_update(
    guard: GuardUserStaff,
    user: Json<user_schema::UpdateUser>,
) -> (
    Status,
    Result<Json<user_schema::User>, Json<schema::ErrorResult>>,
) {
    match user_usecase::update(guard.user.claims.id, true, &user.0).await {
        Ok(v) => (Status::Ok, Ok(Json(v))),
        Err(e) => match e {
            user_usecase::ErrorUpdate::UserNotFound => (
                Status::NotFound,
                Err(Json(schema::ErrorResult {
                    err_type: schema::ErrorType::NotFound,
                    err_msg: "user doesn't exist".to_string(),
                    err_detail: None,
                })),
            ),
        },
    }
}

#[openapi(tag = "User Staff")]
#[guard_permission(error_ty = GuardError, perm_error = MissingPermission, all_perms = [user_schema::StaffPermission::UpdateStaffUser])]
#[put("/<user_id>", data = "<user>")]
pub async fn update(
    _guard: GuardUserStaff,
    user_id: Uuid,
    user: Json<user_schema::UpdateUser>,
) -> (
    Status,
    Result<Json<user_schema::User>, Json<schema::ErrorResult>>,
) {
    match user_usecase::update(user_id, true, &user.0).await {
        Ok(v) => (Status::Ok, Ok(Json(v))),
        Err(e) => match e {
            user_usecase::ErrorUpdate::UserNotFound => (
                Status::NotFound,
                Err(Json(schema::ErrorResult {
                    err_type: schema::ErrorType::NotFound,
                    err_msg: "user doesn't exist".to_string(),
                    err_detail: None,
                })),
            ),
        },
    }
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    merdge_mulit_routes![settings, [get_multiple, create, update]]
}
