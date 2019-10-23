use crate::model::{ErrorResponse, Order};
use actix_web::web::{Data, HttpResponse, Json};
use daprs_core::state::{StateClient, Stateful};
use http::StatusCode;
use log::error;

pub fn post(json_order: Json<Order>, state_client: Data<StateClient>) -> HttpResponse {
    let order = json_order.into_inner();
    if let Err(e) = state_client.push(order.clone()) { 
        error!("Error pushing state for key: {} value: {:?}, received: {}", Order::key(), order, e);
        ErrorResponse::from(e).into()
    } else {
        HttpResponse::build(StatusCode::ACCEPTED).finish()
    }
}

pub fn get(state_client: Data<StateClient>) -> HttpResponse {
    match state_client.get::<Order>() {
        Ok(order) => HttpResponse::build(StatusCode::OK).json(order),
        Err(e) => {
            error!("Error retrieving state for key {}, received {}", Order::key(), e);
            ErrorResponse::from(e).into()
        }
    }
}
