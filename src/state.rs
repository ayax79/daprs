use crate::error::DaprError;
use log::{debug, error};
use reqwest;
use serde::{Deserialize, Serialize};

/// Client for persisting and retrieving state from the Dapr state service
#[derive(Clone, Serialize)]
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
        StateClient { state_url }
    }

    /// Save the state for the specified object type
    /// The type passed in must implement serde Serialize
    pub async fn save<S: Serialize>(&self, key: &str, value: S) -> Result<(), DaprError> {
        let state = vec![State::new(key, value)];
        let response = Client::new()
            .post(&self.state_url)
            .json(&state)
            .send()
            .await?;
        // If we were non-success make that an error too
        let response = response.error_for_status()?;
        if !response.status().is_success() {
            error!(
                "Receiving error response from server: {:?} body: {:?}",
                response.status(),
                response.text().await?
            );
        }

        Ok(())
    }

    /// Gets the last state set for the specified object
    /// The type retrieved must implement serde Deserialize
    pub async fn get<S>(&self, key: &str) -> Result<S, DaprError>
    where
        for<'de> S: Deserialize<'de>,
    {
        let url = self.url(key);
        let response = Client::new().get(&url).send().await?;
        let response = response.error_for_status()?; // If we were non-success make that an error too
        response
            .json::<S>() // Attempt to deserialize json
            .await
            .map_err(|e| DaprError::from_get(key, e)) // Coerce the reqwest error to a DaprError
    }

    fn url(&self, key: &str) -> String {
        format!("{}/{}", self.state_url, key)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct State<S: Serialize> {
    key: String,
    value: S,
}

impl<S: Serialize> State<S> {
    fn new(key: &str, s: S) -> Self {
        State {
            key: key.to_owned(),
            value: s,
        }
    }
}
