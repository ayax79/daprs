use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct FooMessage {
    message: String,
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize() {
        let msg = "Message for the foo topic";
        let json = format!("{{\"message\": \"{}\"}}", msg);
        let foo_message: FooMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(msg, foo_message.message);
    }
}
