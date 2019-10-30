use crate::model::FooMessage;
use actix_web::web::{HttpRequest, HttpResponse, Json, Payload};
use actix_web::{middleware, App, FromRequest};
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
    use super::*;
    use crate::init_logger;
    use actix_service::Service;
    use actix_web::test::TestRequest;
    use actix_web::web::JsonConfig;
    use actix_web::{error, http::StatusCode, test, web, App, HttpResponse};
    use bytes::Bytes;
    use http::header;
    use mime;
    use std::str::FromStr;

    #[test]
    fn test_webevent_json() {
        let payload = b"{\"message\": \"Message for the foo topic\"}";

        let (req, mut pl) = TestRequest::with_header(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/webevents+json"),
        )
        .set_payload(Bytes::from_static(payload))
        .data(JsonConfig::default().content_type(|mime: mime::Mime| {
            mime == mime::Mime::from_str("application/webevents+json").unwrap()
        }))
        .to_http_parts();
        let s = test::block_on(Json::<FooMessage>::from_request(&req, &mut pl));

        assert!(s.is_ok());
    }

    #[test]
    fn test_foo_post() {
        init_logger();

        let payload = b"{\"message\": \"Message for the foo topic\"}";
        let mut app = test::init_service(
            App::new().wrap(middleware::Logger::default()).service(
                web::resource("/test")
                    .data(web::Json::<FooMessage>::configure(|cfg| {
                        cfg.limit(4096)
                            .content_type(|mime| {
                                mime == mime::Mime::from_str("application/webevents+json").unwrap()
                            })
                    }))
                    .to(foo_post),
            ),
        );

        // Create request object
        let req = test::TestRequest::with_uri("/test")
            .header(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/webevents+json"),
            )
            .set_payload(Bytes::from_static(payload))
            .to_request();

        // Execute application
        let mut resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
