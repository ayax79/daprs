pub mod error;
pub mod state;

use crate::error::DaprError;
use std::env;

#[cfg(feature = "actix")]
pub mod actix;

/// Looks for the port number that dapper is running on in the environment variable DAPPER_HTTP_PORT.
/// If the environment variable is not defined or cannot be parsed coerce to a DaprError
pub fn dapper_http_port() -> Result<u16, DaprError> {
    let port_string = env::var("DAPR_HTTP_PORT")?;
    Ok(port_string.parse::<u16>()?)
}
