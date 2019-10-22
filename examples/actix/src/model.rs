use actix_web::HttpResponse;
use daprs_core::error::DaprError;
use daprs_core::state::Stateful;
use daprs_derive::Stateful;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::convert::{From, Into};
use uuid::Uuid;

#[derive(Stateful, Serialize, Deserialize, Debug)]
pub struct Order {
    order_id: Uuid,
    product: String,
    quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    message: String,
}

impl From<DaprError> for ErrorResponse {
    fn from(e: DaprError) -> Self {
        ErrorResponse {
            message: format!("{}", e),
        }
    }
}

impl Into<HttpResponse> for ErrorResponse {
    fn into(self) -> HttpResponse {
        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(self)
    }
}
