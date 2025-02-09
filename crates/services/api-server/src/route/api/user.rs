use crate::{
    guard::{staff::user::UserStaff as GuardUserStaff, GuardError},
    merdge_mulit_routes,
    query::user as user_query,
    schema::user as user_schema,
    usecase::user as user_usecase,
};
use rocket::serde::json::Json;
use rocket_okapi::{okapi::openapi3::OpenApi, openapi, settings::OpenApiSettings};
use util_lib::guard::guard_permission;

#[openapi(tag = "User")]
#[guard_permission(error_ty = GuardError, perm_error = MissingPermission)]
#[get("/?<req_query..>")]
pub async fn get_multiple(
    guard: GuardUserStaff,
    req_query: user_query::User,
) -> Json<user_schema::UserList> {
    Json(user_usecase::get_all(req_query).await)
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    merdge_mulit_routes![settings, [get_multiple]]
}
