//use crate::error::{Error, Result};
use poem::{error::BadRequest, handler, http::StatusCode, web::Query, Result};
use std::collections::HashMap;
use uuid::Uuid;

use crate::configuration::get_configuration;
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

/*
https://sso.codegene.xyz/api/login/oauth/access_token?grant_type=authorization_code&client_id=66e22804c3d5772b80d6&client_secret=40f6e13382ec381103ce5c7253fdbf4a869201c1&code=e5dd6086027216e45c93' \
*/

#[handler]
pub async fn sso_cb(
    Query(SsoLoginCallbackParam { code, state }): Query<SsoLoginCallbackParam>,
) -> Result<StatusCode> {
    println!("code: {}, state: {}", code, state);
    let client = reqwest::Client::new();
    let setting = get_configuration().unwrap();

    let token_res = client
        .post(format!(
            "https://sso.codegene.xyz/api/login/oauth/access_token?grant_type=authorization_code&client_id={}&client_secret={}&code={}", setting.sso.client_id, setting.sso.client_secret, code
        ))
        .send()
        .await
        .map_err(BadRequest)?
        .json::<SsoTokenResponse>()
        .await
        .map_err(BadRequest)?;

    println!("sso response: {:?}", token_res);

    Ok(StatusCode::OK)
}
