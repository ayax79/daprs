use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct FooMessage {
    message: String,
}
