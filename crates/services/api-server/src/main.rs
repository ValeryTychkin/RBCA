mod guard;
mod query;
mod route;
mod schema;
mod usecase;

use migration::init as init_migration;

#[macro_use]
extern crate rocket;

/// Macro to merge Rocket routes with OpenAPI and/or without.
///
/// - With only OpenAPI routes:
///   ```rust
///   merdge_mulit_routes!(settings, [route_with_openapi1, route_with_openapi2]);
///   ```
///
/// - With both OpenAPI routes and additional standard Rocket routes:
///   ```rust
///   merdge_mulit_routes!(settings, [route_with_openapi1, route_with_openapi2], [regular_route1, regular_route2]);
///   ```
#[macro_export]
macro_rules! merdge_mulit_routes {
    ($settings:ident,[ $( $route_with_openapi:expr ),* ] ) => {{
        rocket_okapi::openapi_get_routes_spec![$settings: $($route_with_openapi),*]
    }};
    ($settings:ident, [ $( $route_with_openapi:expr ),* ], [ $( $route:expr ),* ] ) => {{
        let default_routes = rocket::routes![$($route),*];
        let (openapi_routes, openapi_struct) = rocket_okapi::openapi_get_routes_spec![$settings: $($route_with_openapi),*];
        ([default_routes, openapi_routes].concat(), openapi_struct)
    }};
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    init_migration().await;

    let mut _rocket = rocket::build();

    _rocket = route::init_routes(_rocket);

    _rocket.launch().await?;

    Ok(())
}
