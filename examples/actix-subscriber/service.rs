use crate::model::FooMessage;
use actix_web::web::{HttpResponse, Json};
use actix_web::{middleware, App, FromRequest};
use daprs::actix::EventEnvelope;
use http::StatusCode;
use log::info;
use serde_json::{json, Value as JValue};

pub fn subscribe() -> HttpResponse {
    info!("subscribe called, returning topics name: foo, bar");
    HttpResponse::build(StatusCode::OK).json(json!(vec!["foo", "bar"]))
}

pub fn foo_post(foo: Json<EventEnvelope<FooMessage>>) -> HttpResponse {
    info!("Received message: {:?}", foo);
    HttpResponse::build(StatusCode::OK).finish()
}