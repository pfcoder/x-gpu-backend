use crate::configuration::get_configuration;
use poem::{web::Data, Endpoint, Request};
use poem_openapi::{
    auth::ApiKey, param::Path, payload::Json, ApiResponse, OpenApi, SecurityScheme,
};
use sqlx::PgPool;

pub struct GpuApi;

#[OpenApi]
impl GpuApi {}
