use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use time::Date;
use util_lib::{
    auth::jwt::Oauth2TokenClaims, date::schema::date_rfc3339, jwt, string::validate::string_1_255,
};
use uuid::Uuid;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Login {
    #[serde(deserialize_with = "string_1_255")]
    pub email: String,
    #[serde(deserialize_with = "string_1_255")]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelfUserTokenClaims {
    pub id: Uuid,
    #[serde(flatten)]
    pub oauth2_claims: Oauth2TokenClaims,
}

impl SelfUserTokenClaims {
    pub fn get_key_for_cache(&self) -> String {
        get_key_for_cache(self.id.to_string(), self.oauth2_claims.jti.to_string())
    }

    pub fn get_prefix_key_for_cache(&self) -> String {
        get_prefix_key_for_cache(self.id.to_string())
    }

    pub fn from_string(token_str: &String) -> jwt::JWTResult<Self> {
        match jwt::decode::<Self>(&token_str) {
            Ok(token) => Ok(token.claims),
            Err(e) => Err(e),
        }
    }
}

pub fn get_key_for_cache(user_id: String, jwt_id: String) -> String {
    format!("USER:{}_JWT:{}", user_id, jwt_id)
}

pub fn get_prefix_key_for_cache(user_id: String) -> String {
    format!("USER:{}_JWT:*", user_id)
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct Register {
    #[serde(deserialize_with = "string_1_255")]
    pub name: String,
    #[serde(deserialize_with = "string_1_255")]
    pub email: String,
    #[serde(deserialize_with = "string_1_255")]
    pub password: String,
    #[schemars(schema_with = "date_rfc3339")]
    pub birthday: Date,
    pub organization_id: Uuid,
}
