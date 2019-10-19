use reqwest;
use serde::{Deserialize, Serialize};
use crate::error::DaprError;

/// Trait representing any state that can be stored
pub trait Stateful : Sized {
    fn key() -> &'static str;
}

/// Client for persisting and retrieving state from the Dapr state service
#[derive(Clone)]
pub struct StateClient {
    state_url: String
}

impl StateClient {

    pub fn new(port: u16) -> Self {
        StateClient {
            state_url: format!("http://localhost:{}/v1.0/state", port)
        }
    }

    /// Save the state for the specified object type
    /// The type passed in must implement serde Serialize
    pub fn push<S>(&self, s: S) -> Result<(), DaprError> 
    where S: Stateful + Serialize {
        let state = State::new(s);
        reqwest::Client::new()
            .post(&self.state_url)
            .json(&state)
            .send()
            .and_then(|r| r.error_for_status())     // If we were non-success make that an error too
            .map(|_| ())                            // Empty return coerce
            .map_err(DaprError::from_send)
    }

    /// Gets the last state set for the specified object
    /// The type retrieved must implement serde Deserialize
    pub fn get<S>(&self) -> Result<S, DaprError>
    where for<'de> S: Stateful + Deserialize<'de> {
        let url = self.url::<S>();
        reqwest::Client::new()
            .get(&url)
            .send()
            .and_then(|r| r.error_for_status())     // If we were non-success make that an error too
            .and_then(|mut r| r.json::<S>())        // Attempt to deserialize json
            .map_err(DaprError::from_get)           // Coerce the reqwest error to a DaprError 
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

impl <S: Stateful> State<S> {
    fn new(s: S) -> Self {
        State {
            key: S::key().to_owned(),
            value: s
        }
    }
}