//use crate::error::{Error, Result};
use chrono::{Duration, Utc};
use poem::{
    error::BadRequest, handler, http::StatusCode, http::Uri, session::Session, web::Data,
    web::Query, web::Redirect, Request, Result,
};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::configuration::get_configuration;
use crate::model::CreateUserData;
use crate::service::user::UserService;
use crate::utils::jwt::sign;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SsoLoginCallbackParam {
    pub code: String,
    pub state: String,
}

#[derive(Deserialize, Debug)]
struct SsoTokenResponse {
    access_token: String,
    id_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: usize,
    scope: String,
}

#[derive(Deserialize, Debug)]
struct SsoUserInfoResponse {
    sub: String,
    iss: String,
    aud: String,
}

#[handler]
pub async fn sso_cb(
    session: &Session,
    pool: Data<&PgPool>,
    Query(SsoLoginCallbackParam { code, state }): Query<SsoLoginCallbackParam>,
) -> Result<Redirect> {
    println!("code: {}, state: {}", code, state);
    let client = reqwest::Client::new();
    let setting = get_configuration().unwrap();

    let token_res = client
        .post(format!(
            "https://sso.codegene.xyz/api/login/oauth/access_token?grant_type=authorization_code&client_id={}&client_secret={}&code={}", 
            setting.sso.client_id, setting.sso.client_secret, code
        ))
        .send()
        .await
        .map_err(BadRequest)?
        .json::<SsoTokenResponse>()
        .await
        .map_err(BadRequest)?;

    tracing::info!("sso response: {:?}", token_res);
    // get user info
    // https://sso.codegene.xyz/api/userinfo
    let user_info = client
        .get(format!(
            "https://sso.codegene.xyz/api/userinfo?accessToken={}",
            token_res.access_token
        ))
        .send()
        .await
        .map_err(BadRequest)?
        .json::<SsoUserInfoResponse>()
        .await
        .map_err(BadRequest)?;

    tracing::info!("user info: {:?}", user_info);

    // check if user exists, update db
    // if not, create user
    UserService::create(
        pool.0,
        CreateUserData {
            id: user_info.sub.clone(),
            access_token: token_res.access_token,
            refresh_token: token_res.refresh_token,
            expires_at: Utc::now() + Duration::seconds(token_res.expires_in as i64),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        },
    )
    .await
    .map_err(BadRequest)?;

    // give user back access token, mark as logged in, update cookie
    let token = sign(Uuid::parse_str(&user_info.sub).map_err(BadRequest)?).map_err(BadRequest)?;

    session.set("Authorization", token);
    //tracing::info!("new jwt token: {:?}", token);
    // redirect to home

    Ok(Redirect::see_other(Uri::from_static(
        "https://cloud.codegene.xyz/",
    )))
}
