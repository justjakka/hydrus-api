pub mod client;
pub mod traits;

use thiserror::Error;

/// Error wrapper
#[derive(Error, Debug)]
pub enum HydrusError {
    #[error("failed to connect to Hydrus")]
    NetworkError(reqwest::Error),
    #[error("failed to encode/Deserialize data")]
    DeserializeError(serde_json::Error),
    #[error("io error")]
    IOError(std::io::Error),
    #[error("api or session key needed")]
    KeyNotSupplied,
}

impl From<serde_json::Error> for HydrusError {
    fn from(value: serde_json::Error) -> Self {
        HydrusError::DeserializeError(value)
    }
}

impl From<std::io::Error> for HydrusError {
    fn from(value: std::io::Error) -> Self {
        HydrusError::IOError(value)
    }
}

impl From<reqwest::Error> for HydrusError {
    fn from(value: reqwest::Error) -> Self {
        HydrusError::NetworkError(value)
    }
}
