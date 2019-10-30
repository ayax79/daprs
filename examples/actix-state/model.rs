use actix_web::HttpResponse;
use daprs::error::DaprError;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::convert::{From, Into};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    order_id: Uuid,
    product: String,
    quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    message: String,
    #[serde(skip_serializing)]
    status: u16,
}

impl From<DaprError> for ErrorResponse {
    fn from(e: DaprError) -> Self {
        match e {
            DaprError::NotFoundError(ref key) => ErrorResponse {
                message: format!("key: {}. {}", key, e),
                status: 404,
            },
            _ => ErrorResponse {
                message: format!("{}", e),
                status: 500,
            },
        }
    }
}

impl Into<HttpResponse> for ErrorResponse {
    fn into(self) -> HttpResponse {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        HttpResponse::build(status).json(self)
    }
}
