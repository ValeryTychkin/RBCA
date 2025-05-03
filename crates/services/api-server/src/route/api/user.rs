use crate::{
    guard::{staff::user::UserStaff as GuardUserStaff, GuardError},
    merdge_mulit_routes,
    query::user as user_query,
    schema::user as user_schema,
    usecase::user as user_usecase,
};
use rocket::serde::json::Json;
use rocket_okapi::{
    get_nested_endpoints_and_docs, okapi::openapi3::OpenApi, openapi, settings::OpenApiSettings,
};
use rocket_util_lib::guard_permission;

#[openapi(tag = "User")]
#[guard_permission(error_ty = GuardError, perm_error = MissingPermission)]
#[get("/?<req_query..>")]
pub async fn get_multiple(
    _guard: GuardUserStaff,
    req_query: user_query::User,
) -> Json<user_schema::UserList> {
    let mut req_query = req_query;
    req_query.is_staff = Some(false);
    Json(user_usecase::get_all(&req_query).await)
}

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    get_nested_endpoints_and_docs! {
        "/" => merdge_mulit_routes![settings, [get_multiple]],
    }
}
