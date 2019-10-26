use crate::model::FooMessage;
use actix_web::web::{HttpRequest, HttpResponse, Json, Payload};
use actix_web::FromRequest;
use http::StatusCode;
use log::info;
use serde_json::json;

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

#[cfg(test)]
mod tests {
    use actix_service::Service;
    use actix_web::test::TestRequest;
    use actix_web::{http::StatusCode, test, web, App, HttpResponse};

    #[test]
    fn test_foo_post() {
        let payload = b"{\"message\": \"Message for the foo topic\"}";
        let mut app = test::init_service(App::new().service(web::resource("/test").to(foo_post)));

        // Create request object
        let req = test::TestRequest::with_uri("/test")
            .set_payload(payload)
            .to_request();

        // Execute application
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
