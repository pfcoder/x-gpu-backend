#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

use futures::Future;
use sqlx::PgPool;

use poem::{
    endpoint::StaticFilesEndpoint,
    listener::TcpListener,
    middleware::Cors,
    post,
    session::{CookieConfig, CookieSession},
    web::Data,
    Endpoint, EndpointExt, Result, Route, Server,
};
use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    ApiResponse, Object, OpenApi, OpenApiService,
};

use crate::configuration::get_configuration;

mod api;
pub mod configuration;
mod dto;
mod error;
//mod extractors;
mod handlers;
mod model;
mod response;
mod service;
mod sql;
pub mod telemetry;
mod utils;

pub mod constants;

fn app(pg_pool: PgPool) -> impl Endpoint {
    /*let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(AddExtensionLayer::new(pg_pool))
        .into_inner();

    let auth_api = Router::new()
        .route("/login", post(handlers::user::login))
        .route("/register", post(handlers::user::register));
    let vc_api = Router::new().route("/template", post(handlers::vc::vc_tplt_create));
    let did_api = Router::new().route("/create", post(handlers::did::did_create));

    Router::new()
        .route("/api/:v/health_check", get(handlers::health_check))
        .nest("/api/:v/auth", auth_api)
        .nest("/api/:v/vc", vc_api)
        .nest("/api/:v/did", did_api)
        .layer(middleware_stack)*/
    //let server_key = Hmac::<Sha256>::new_from_slice(SERVER_KEY).expect("valid server key");
    let api_service =
        OpenApiService::new(api::GpuApi, "GPU Api", "1.0.0").server("X-GPU API Server");
    let ui = api_service.swagger_ui();
    let spec = api_service.spec();
    Route::new()
        .nest("/api", api_service)
        .nest("/api/ui", ui)
        .nest(
            "/api/spec",
            poem::endpoint::make_sync(move |_| spec.clone()),
        )
        .nest("/sso/cb", handlers::user::sso_cb)
        .nest("/health_check", handlers::health_check)
        .nest(
            "/",
            StaticFilesEndpoint::new(get_configuration().unwrap().front_path)
                .index_file("index.html"),
        )
        .with(Cors::new())
        .with(CookieSession::new(CookieConfig::default().secure(false)))
        .data(pg_pool)
}

/// Provide database connection, and TCP listener, this can be different in production build and test build
pub fn server(
    pg_pool: PgPool,
    listener: TcpListener<String>,
) -> impl Future<Output = std::result::Result<(), std::io::Error>> {
    /*axum::Server::from_tcp(listener)
    .unwrap()
    .serve(app(pg_pool).into_make_service())*/

    Server::new(listener).run(app(pg_pool))
}
