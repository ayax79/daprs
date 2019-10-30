use crate::error::DaprError;
use reqwest;
use serde::{Deserialize, Serialize};
use log::{debug, error};

/// Client for persisting and retrieving state from the Dapr state service
#[derive(Clone)]
pub struct PublishClient {
    pub_url: String,
}

impl PublishClient {

    /// Creates a new instance of PublishClient
    /// 
    /// # Arguments
    /// * `dapr_port` - The http port that dapr is listening on
    pub fn new(dapr_port: u16) -> Self {
        let pub_url = format!("http://localhost:{}/v1.0/publish", dapr_port);
        debug!("Configuring state client with state url: {}", pub_url);
        PublishClient {
            pub_url, 
        }
    }

    /// Save the state for the specified object type
    /// The type passed in must implement serde Serialize
    pub fn save<S: Serialize>(&self, topic: &str, value: S) -> Result<(), DaprError> {
        let state = vec![State::new(topic, value)];
        reqwest::Client::new()
            .post(&self.pub_url)
            .json(&state)
            .send()
            .and_then(|mut r| {
                if !r.status().is_success() {
                    error!("Receiving error response from server: {:?} body: {:?}", r.status(), r.text());                
                }
                // If we were non-success make that an error too
                r.error_for_status()
            }) 
            .map(|_| ()) // Empty return coerce
            .map_err(DaprError::from_send)
    }

    /// Gets the last state set for the specified object
    /// The type retrieved must implement serde Deserialize
    pub fn get<S>(&self, topic: &str) -> Result<S, DaprError>
    where
        for<'de> S: Deserialize<'de>,
    {
        let url = self.url(topic);
        reqwest::Client::new()
            .get(&url)
            .send()
            .and_then(|r| r.error_for_status()) // If we were non-success make that an error too
            .and_then(|mut r| r.json::<S>()) // Attempt to deserialize json
            .map_err(|e| DaprError::from_get(topic, e)) // Coerce the reqwest error to a DaprError
    }

    fn url(&self, topic: &str) -> String {
        format!("{}/{}", self.pub_url, topic)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct State<S: Serialize> {
    topic: String,
    value: S,
}

impl<S: Serialize> State<S> {
    fn new(topic: &str, s: S) -> Self {
        State {
            topic: topic.to_owned(),
            value: s,
        }
    }
}
