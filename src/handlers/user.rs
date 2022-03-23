use std::collections::HashMap;

use poem::{handler, http::StatusCode, web::Query, Result};
use serde_json::json;
use uuid::Uuid;

use crate::configuration::get_configuration;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SsoLoginCallbackParam {
    pub code: String,
    pub state: String,
}

#[tracing:: instrument(
    name = "sso callback",
    fields(request_id = %Uuid::new_v4())
)]
#[handler]
pub async fn sso_cb(
    Query(SsoLoginCallbackParam { code, state }): Query<SsoLoginCallbackParam>,
) -> StatusCode {
    println!("code: {}, state: {}", code, state);
    let client = reqwest::Client::new();
    let setting = get_configuration().unwrap();

    let mut map = HashMap::new();
    map.insert("grant_type", "authorization_code");
    map.insert("client_id", &setting.sso.client_id);
    map.insert("client_secret", &setting.sso.client_secret);
    map.insert("code", &code);

    let resp = client
        .post("https://sso.codegene.xyz/api/login/oauth/access_token")
        .json(&map)
        .send()
        .await
        .unwrap();

    println!("sso response: {:?}", resp);

    StatusCode::OK
}
