use crate::model::FooMessage;
use actix_web::web::{HttpRequest, HttpResponse, Json, Payload};
use http::StatusCode;
use log::info;
use serde_json::json;
use actix_web::FromRequest;

pub fn subscribe() -> HttpResponse {
    info!("subscribe called, returning topic name: foo");
    HttpResponse::build(StatusCode::OK).json(json!(vec!["foo", "bar"]))
}

pub fn foo_post(foo: Json<FooMessage>) -> HttpResponse {
    info!("Received message: {:?}", foo);
    HttpResponse::build(StatusCode::OK).finish()
}

pub fn bar_post(req: HttpRequest, payload: Payload) -> HttpResponse {
    info!("Received request: {:?}", req);
    // let json = Json::<FooMessage>::from_request(&req, &mut payload);
    // info!("json: {}", json)
    HttpResponse::build(StatusCode::OK).finish()
}
