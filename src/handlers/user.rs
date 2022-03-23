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

#[handler]
pub async fn sso_cb(
    Query(SsoLoginCallbackParam { code, state }): Query<SsoLoginCallbackParam>,
) -> Result<StatusCode> {
    println!("code: {}, state: {}", code, state);
    let client = reqwest::Client::new();
    let setting = get_configuration().unwrap();

    let mut map = HashMap::new();
    map.insert("grant_type", "authorization_code");
    map.insert("client_id", &setting.sso.client_id);
    map.insert("client_secret", &setting.sso.client_secret);
    map.insert("code", &code);

    let token_res = client
        .post("https://sso.codegene.xyz/api/login/oauth/access_token")
        .json(&map)
        .send()
        .await
        .map_err(BadRequest)?
        .json::<SsoTokenResponse>()
        .await
        .map_err(BadRequest)?;

    println!("sso response: {:?}", token_res);

    Ok(StatusCode::OK)
}
