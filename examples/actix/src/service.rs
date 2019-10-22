use crate::model::Order;
use actix_web::{
    Error as AWError,
    web::{Data, Json, HttpResponse}
};
use daprs_core::{
    error::DaprError,
    state::StateClient
}

pub fn post(json_order: Json<Order>, state_client: Data<StateClient>) -> HttpResponse {
    let order = json_order.into_inner();
    if let Err(e) = state_client.push(order) {
        HttpResponse::from_error(error: Error)

    }
    else {

    }
}