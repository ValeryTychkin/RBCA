use crate::schema::{auth as auth_schema, base as base_schema, user as user_schema};
use repository_db_lib::user::{user_entity, Repository, User as UserRep};
use repository_redis_lib as redis_repository;
use sea_orm::{ActiveValue, ColumnTrait, Condition};
use util_lib::{
    auth::{self, jwt as auth_jwt},
    jwt::encode as jwt_encode,
};

pub enum ErrorLogin {
    Email,
    Password,
}

pub async fn login(
    user_login: auth_schema::Login,
) -> Result<auth_jwt::Oauth2LoginResult, ErrorLogin> {
    // Get filter
    let filter = Condition::all().add(user_entity::Column::Email.eq(user_login.email.to_owned()));
    // Try to get User
    let rep = UserRep::new().await;
    let user: user_entity::Model;
    match rep.get_one(Some(filter)).await.unwrap() {
        Some(v) => user = v,
        None => {
            return Err(ErrorLogin::Email);
        }
    }
    // Check User password
    if !user.is_valid_password(user_login.password.as_str()) {
        return Err(ErrorLogin::Password);
    }
    // Get access and refresh user claims
    let (access_user_claims, refresh_user_claims) =
        auth_schema::SelfUserTokenClaims::acc_and_ref_from_model(&user);
    // Save user claims in cache
    save_token(&access_user_claims).await;
    save_token(&refresh_user_claims).await;
    // Convert claims into jwt and return
    Ok(auth_jwt::Oauth2LoginResult {
        access_token: jwt_encode(&access_user_claims).unwrap(),
        refresh_token: jwt_encode(&refresh_user_claims).unwrap(),
        token_type: auth::TokenType::Bearer,
        expires_in: access_user_claims.oauth2_claims.get_life_sec(),
    })
}

pub async fn logout(user_claims: auth_schema::SelfUserTokenClaims) {
    del_acc_ref_tokens(&user_claims).await;
}

pub async fn introspect(token_intro: auth_jwt::IntrospectInput) -> auth_jwt::IntrospectResult {
    // Try to deserialize claims from jwt
    let user_claims: auth_schema::SelfUserTokenClaims;
    match auth_schema::SelfUserTokenClaims::from_jwt(&token_intro.token) {
        Ok(v) => user_claims = v,
        Err(_) => {
            return auth_jwt::IntrospectResult {
                active: false,
                ..Default::default()
            };
        }
    }
    // Ceck match with token type (if token type was send)
    if let Some(_) = &token_intro.token_type_hint {
        if !(token_intro.is_access() == user_claims.oauth2_claims.is_access()) {
            return auth_jwt::IntrospectResult {
                active: false,
                ..Default::default()
            };
        }
    }
    // Check lifetime range
    if let Err(_) = user_claims.oauth2_claims.validate_date_range() {
        return auth_jwt::IntrospectResult {
            active: false,
            ..Default::default()
        };
    }
    // Check token not revoked (on exist)
    if !token_is_exist(&user_claims).await {
        return auth_jwt::IntrospectResult {
            active: false,
            ..Default::default()
        };
    }
    // Return result
    auth_jwt::IntrospectResult {
        active: true,
        ..Default::default()
    }
}

pub async fn registration(
    user_reg: auth_schema::Register,
) -> Result<user_schema::User, base_schema::ErrorResult> {
    // Check if email allready exist
    let filter = Condition::all().add(user_entity::Column::Email.eq(user_reg.email.to_owned()));
    let rep = UserRep::new().await;
    if let Some(_) = rep.get_one(Some(filter)).await.unwrap() {
        return Err(base_schema::ErrorResult {
            err_msg: "email allready used".to_string(),
            err_detail: None,
        });
    }
    // Save new user
    let active_model = user_entity::ActiveModel {
        name: ActiveValue::Set(user_reg.name),
        email: ActiveValue::Set(user_reg.email),
        password: ActiveValue::Set(user_reg.password),
        birthday: ActiveValue::Set(user_reg.birthday),
        ..Default::default()
    };
    // Convert Model into Schema
    let model = rep.create(active_model).await.unwrap();
    Ok(user_schema::User {
        id: model.id,
        name: model.name,
        email: model.email,
        is_deleted: model.is_deleted,
        updated_ad: model.updated_at,
        created_at: model.created_at,
    })
}

pub async fn save_token(token_claims: &auth_schema::SelfUserTokenClaims) {
    redis_repository::set(
        token_claims.get_key_for_cache(),
        serde_json::to_string(&token_claims).unwrap(),
        Some(token_claims.oauth2_claims.get_life_sec()),
    )
    .await;
}

pub async fn token_is_exist(token_claims: &auth_schema::SelfUserTokenClaims) -> bool {
    redis_repository::exist(token_claims.get_key_for_cache()).await
}

pub async fn del_acc_ref_tokens(token_claims: &auth_schema::SelfUserTokenClaims) {
    redis_repository::del(token_claims.get_key_for_cache()).await;
    redis_repository::del(auth_schema::get_key_for_cache(
        token_claims.id.to_string(),
        token_claims.oauth2_claims.sub_jti.to_string(),
    ))
    .await;
}
