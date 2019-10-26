mod model;
mod service;

use crate::service::{bar_post, foo_post, subscribe};
use actix_web::FromRequest;
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use daprs::error::DaprError;
use log::{error, info};
use mime;
use model::FooMessage;
use pretty_env_logger;
use std::env;
use std::process::exit;

/// Port number this microservice should listen to
const PORT: u16 = 3000;

fn main() {
    init_logger();
    info!("Starting actix-subscriber-example");
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
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/foo")
                    .data(web::Json::<FooMessage>::configure(|cfg| {
                        cfg.limit(4096)
                            .content_type(|mime| {
                                // mime.type_() == mime::APPLICATION
                                //     && mime.subtype().as_str() == "cloudevents+json"
                                true
                            })
                            .error_handler(|err, req| {
                                error!("error: {}", err);
                                error!("request: {:?}", req);

                                // <- create custom error response
                                error::InternalError::from_response(
                                    err,
                                    HttpResponse::Conflict().finish(),
                                )
                                .into()
                            })
                    }))
                    .route(web::post().to(foo_post)), // our topic name
            )
            .service(web::resource("/dapr/subscribe").route(web::get().to(subscribe)))
        // .route("/dapr/subscribe", web::get().to(subscribe))
        // .route("/bar", web::post().to(bar_post))
        // .route("/foo", web::post().to(foo_post))
    })
    .bind(format!("127.0.0.1:{}", PORT))
    .unwrap()
    .run()
    .unwrap();
    Ok(())
}
