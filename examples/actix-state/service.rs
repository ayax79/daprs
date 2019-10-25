use crate::model::{ErrorResponse, Order};
use actix_web::web::{Data, HttpResponse, Json};
use daprs::state::StateClient;
use http::StatusCode;
use log::error;

pub async fn post(json_order: Json<Order>, state_client: Data<StateClient>) -> HttpResponse {
    let order = json_order.into_inner();
    if let Err(e) = state_client.save("order", order.clone()).await {
        error!(
            "Error pushing state for key: order value: {:?}, received: {}",
            order, e
        );
        ErrorResponse::from(e).into()
    } else {
        HttpResponse::build(StatusCode::ACCEPTED).finish()
    }
}

pub async fn get(state_client: Data<StateClient>) -> HttpResponse {
    match state_client.get::<Order>("order").await {
        Ok(order) => HttpResponse::build(StatusCode::OK).json(order),
        Err(e) => {
            error!("Error retrieving state for key: order, received {}", e);
            ErrorResponse::from(e).into()
        }
    }
}
