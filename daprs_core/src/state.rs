use crate::error::DaprError;
use reqwest;
use serde::{Deserialize, Serialize};
use log::debug;

/// Trait representing any state that can be stored
pub trait Stateful: Sized {
    fn key(&self) -> String;
}

/// Client for persisting and retrieving state from the Dapr state service
#[derive(Clone)]
pub struct StateClient {
    state_url: String,
}

impl StateClient {

    /// Creates a new instance of StateClient
    /// 
    /// # Arguments
    /// * `dapr_port` - The http port that dapr is listening on
    pub fn new(dapr_port: u16) -> Self {
        let state_url = format!("http://localhost:{}/v1.0/state", dapr_port);
        debug!("Configuring state client with state url: {}", state_url);
        StateClient {
            state_url, 
        }
    }

    /// Save the state for the specified object type
    /// The type passed in must implement serde Serialize
    pub fn push<S>(&self, key: &str, value: S) -> Result<(), DaprError>
    where
        S: Stateful + Serialize,
    {
        let state = State::new(s);
        reqwest::Client::new()
            .post(&self.state_url)
            .json(&state)
            .send()
            .and_then(|r| r.error_for_status()) // If we were non-success make that an error too
            .map(|_| ()) // Empty return coerce
            .map_err(DaprError::from_send)
    }

    /// Gets the last state set for the specified object
    /// The type retrieved must implement serde Deserialize
    pub fn get<S>(&self, key: &str) -> Result<S, DaprError>
    where
        for<'de> S: Stateful + Deserialize<'de>,
    {
        let url = self.url::<S>();
        reqwest::Client::new()
            .get(&url)
            .send()
            .and_then(|r| r.error_for_status()) // If we were non-success make that an error too
            .and_then(|mut r| r.json::<S>()) // Attempt to deserialize json
            .map_err(|e| DaprError::from_get(S::key(), e)) // Coerce the reqwest error to a DaprError
    }

    fn url<S: Stateful>(&self) -> String {
        format!("{}/{}", self.state_url, S::key())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct State<S: Stateful> {
    key: String,
    value: S,
}

impl<S: Stateful> State<S> {
    fn new(s: S) -> Self {
        State {
            key: S::key().to_owned(),
            value: s,
        }
    }
}
