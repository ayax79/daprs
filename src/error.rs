use http::StatusCode;
use reqwest::Error as ReqwestError;
use std::convert::From;
use std::env::VarError;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum DaprError {
    /// Thrown if the specified port is incorrect
    InvalidDaprPort(String),
    /// Misc errors on pushing state to the Dapr state service
    SendStateError(String),
    /// Misc errors on getting from the Dapr state service
    GetStateError(String),
    EmptyStateError(String),
}

impl DaprError {
    pub fn from_get(key: &str, re: ReqwestError) -> Self {
        match re.status() {
            Some(StatusCode::NOT_FOUND) => Self::EmptyStateError(key.to_owned()),
            _ => Self::GetStateError(format!("{}", re)),
        }
    }
}

impl Error for DaprError {}

impl fmt::Display for DaprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDaprPort(ref msg) => write!(f, "InvalidDaprPort: {}", msg),
            Self::SendStateError(ref msg) => write!(f, "Error sending state: {}", msg),
            Self::GetStateError(ref msg) => write!(f, "Error getting state: {}", msg),
            Self::EmptyStateError(ref key) => write!(f, "No stored state for: {}", key),
        }
    }
}

impl From<VarError> for DaprError {
    fn from(ve: VarError) -> Self {
        DaprError::InvalidDaprPort(format!("{}", ve))
    }
}

impl From<ParseIntError> for DaprError {
    fn from(pie: ParseIntError) -> Self {
        DaprError::InvalidDaprPort(format!("{}", pie))
    }
}

impl From<reqwest::Error> for DaprError {
    fn from(error: reqwest::Error) -> Self {
        DaprError::SendStateError(format!("{}", error))
    }
}
