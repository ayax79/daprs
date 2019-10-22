mod model;
mod service;
mod error;

use daprs_core::{
    dapper_http_port,
    error::DaprError,
    state::StateClient,
};
use actix_web::{web, App, HttpServer};
use actix::prelude::*;
use std::process::exit;

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
            // .route("/", web::get().to(index))
            // .route("/again", web::get().to(index2))
    })
    .bind(format!("127.0.0.1:{}", PORT))
    .unwrap()
    .run()
    .unwrap();
    Ok(())
}