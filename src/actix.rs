use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EventEnvelope<S: Sized> {
    pub data: S,
    #[serde(rename = "datacontenttype")]
    pub data_content_type: String,
    pub id: String,
    pub source: String,
    #[serde(rename = "specversion")]
    pub spec_version: String,
    #[serde(rename = "type")]
    pub event_type: String,
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[derive(Deserialize)]
    struct MyJson {
        msg: String
    }

    #[test]
    fn test_envelope() {
        let json = r#"
        {
            "data": { "msg": "hello" },
            "datacontenttype": "application/json", 
            "id": "3ddda715-8f8d-4356-8dd2-8f85d771abca",
            "source": "actix-subscriber-example", 
            "specversion": "0.3", 
            "type": "com.dapr.event.sent"
        }
        "#;

        let result: EventEnvelope<MyJson> = serde_json::from_str(json).unwrap();

        assert_eq!("hello", result.data.msg);
        assert_eq!("application/json", result.data_content_type);
        assert_eq!("3ddda715-8f8d-4356-8dd2-8f85d771abca", result.id);
        assert_eq!("actix-subscriber-example", result.source);
        assert_eq!("0.3", result.spec_version);
        assert_eq!("com.dapr.event.sent", result.event_type);
    }

}