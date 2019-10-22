pub mod error;
pub mod state;

use crate::error::DaprError;
use std::env;

/// Looks for the port number that dapper is running on in the environment variable DAPPER_HTTP_PORT.
/// If the environment variable is not defined or cannot be parsed coerce to a DaprError
pub fn dapper_http_port() -> Result<u16, DaprError> {
    env::var("DAPPER_HTTP_PORT")
        .map_err(DaprError::from)
        .and_then(|port_string| port_string.parse::<u16>().map_err(DaprError::from))
}
