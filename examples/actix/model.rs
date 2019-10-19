use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::convert::From;
use crate::state::Stateful;

#[derive(Stateful, Serialize, Deserialize, Debug)]
pub struct Order {
    order_id: Uuid,
    product: String,
    quantity: i32,
}

impl Stateful for Sized {

}

impl From<Order> for State<Order> {
    fn from(o: Order) -> Self {
        State {
            key: "order".to_owned(),
            value: o,
        }
    }
}


