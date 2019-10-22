mod model;
mod service;

use crate::service::{get, post};
use actix_web::{middleware, web, App, HttpServer};
use daprs_core::{dapper_http_port, error::DaprError, state::StateClient};
use std::process::exit;

/// Port number this microservice should listen to
const PORT: u16 = 3000;

fn main() {
    println!("Starting my rust dapr app");
    if let Err(err) = init() {
        eprintln!("{}", err);
        exit(1);
    }
}

fn init() -> Result<(), DaprError> {
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
