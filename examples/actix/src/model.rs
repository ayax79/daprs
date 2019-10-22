
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use daprs_core::state::Stateful;
use daprs_derive::Stateful;

#[derive(Stateful, Serialize, Deserialize, Debug)]
pub struct Order {
    order_id: Uuid,
    product: String,
    quantity: i32,
}


