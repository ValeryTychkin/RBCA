use rocket::{
    http::Status,
    outcome::{try_outcome, Outcome},
    request::{self, FromRequest, Request},
};
use rocket_okapi::{
    gen::OpenApiGenerator,
    okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData},
    request::{OpenApiFromRequest, RequestHeaderInput},
};

use super::super::{user::User, GuardError};
use crate::schema::user::StaffPermission;

#[derive(Debug)]
pub struct UserStaff {
    pub user: User,
}

impl UserStaff {
    pub async fn get_permissions(&self) -> Vec<StaffPermission> {
        if let Some(permissions) = &self.user.claims.permissions {
            return permissions.to_owned();
        }
        Vec::new()
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for UserStaff {
    type Error = GuardError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user = try_outcome!(request.guard::<User>().await);
        if user.claims.is_staff {
            return Outcome::Success(Self { user });
        }
        Outcome::Error((Status::Forbidden, GuardError::MissingUser))
    }
}

// Implementing OpenApiFromRequest to document the API security requirements in OpenAPI format
impl<'a> OpenApiFromRequest<'a> for UserStaff {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let security_scheme = SecurityScheme {
            description: Some("Requires an Bearer token to access".to_owned()),
            data: SecuritySchemeData::Http {
                scheme: "bearer".to_owned(),
                bearer_format: Some("bearer".to_owned()),
            },
            extensions: Object::default(),
        };
        let mut security_req = SecurityRequirement::new();
        security_req.insert("Authorization".to_owned(), Vec::new());
        Ok(RequestHeaderInput::Security(
            "Authorization".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}
