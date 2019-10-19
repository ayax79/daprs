use std::error::Error;
use std::fmt;
use std::convert::From;
use std::env::VarError;
use std::num::ParseIntError;
use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum DaprError {
    VarErr(String),
    ParseIntError(String),
    SendStateError(String),
    GetStateError(String),
}

impl DaprError {
    pub fn from_send(re: ReqwestError) -> Self {
        DaprError::SendStateError(format!("{}", re))
    }

    pub fn from_get(re: ReqwestError) -> Self {
        DaprError::GetStateError(format!("{}", re))
    }
}

impl Error for DaprError {}

impl fmt::Display for DaprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VarErr(ref msg) => write!(f, "VarErr: {}", msg),
            Self::ParseIntError(ref msg) => write!(f, "ParseIntError: {}", msg),
            Self::SendStateError(ref msg) => write!(f, "Error sending state: {}", msg),
            Self::GetStateError(ref msg) => write!(f, "Error getting state: {}", msg),
        }        
    }
}

impl From<VarError> for DaprError {
    fn from(ve: VarError) -> Self {
        DaprError::VarErr(format!("{}", ve))
    }
}

impl From<ParseIntError> for DaprError {
    fn from(pie: ParseIntError) -> Self {
        DaprError::ParseIntError(format!("{}", pie))
    }
}

