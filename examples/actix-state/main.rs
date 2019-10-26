mod model;
mod service;

use crate::service::{get, post};
use actix_web::{middleware, web, App, HttpServer};
use daprs::{dapper_http_port, error::DaprError, state::StateClient};
use log::info;
use pretty_env_logger;
use std::env;
use std::process::exit;

/// Port number this microservice should listen to
const PORT: u16 = 3000;

fn main() {
    init_logger();
    info!("Starting actix-state-example");
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
                    .route(web::post().to(post))
                    .route(web::get().to(get)),
            )
    })
    .bind(format!("127.0.0.1:{}", PORT))
    .unwrap()
    .run()
    .unwrap();
    Ok(())
}
