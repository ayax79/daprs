mod model;
mod service;

use crate::{
    model::Order,
    service::{get, post},
};
use actix_web::{
    middleware,
    web::{self, Data, Json},
    App, HttpResponse, HttpServer,
};
use daprs::{dapper_http_port, error::DaprError, state::StateClient};
use futures::future::{FutureExt, TryFutureExt};
use futures01::future::Future;
use log::info;
use pretty_env_logger;
use std::env;
use std::process::exit;

/// Port number this microservice should listen to
const PORT: u16 = 3000;

fn main() {
    info!("Starting my rust dapr app");
    init_logger();
    if let Err(err) = init_actix() {
        eprintln!("{}", err);
        exit(1);
    }
}

/// Initialize the logger
/// If there isn't a RUST_LOG environment variable defined we will define one
fn init_logger() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info")
    }
    pretty_env_logger::init();
}

/// Configure an initialize Actix
fn init_actix() -> Result<(), DaprError> {
    let dapper_port = dapper_http_port()?;
    let state_client = StateClient::new(dapper_port);
    HttpServer::new(move || {
        App::new()
            .data(state_client.clone())
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(
                web::resource("/order")
                    .route(web::post().to_async(async_post))
                    .route(web::get().to_async(async_get)),
            )
    })
    .bind(format!("127.0.0.1:{}", PORT))
    .unwrap()
    .run()
    .unwrap();
    Ok(())
}

// This is a temporary ugly conversion from std::futures::Future to the old futures v0.1
// This will go away when actix updates to use std futures.
fn async_post(
    json_order: Json<Order>,
    state_client: Data<StateClient>,
) -> impl Future<Item = HttpResponse, Error = ()> {
    post(json_order, state_client)
        .unit_error()
        .boxed_local()
        .compat()
}
fn async_get(state_client: Data<StateClient>) -> impl Future<Item = HttpResponse, Error = ()> {
    get(state_client).unit_error().boxed_local().compat()
}
