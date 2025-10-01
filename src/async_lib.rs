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

#[cfg(test)]
mod tests {
    use crate::async_lib::{client::*, traits::*};

    #[tokio::test]
    async fn test_service_name_info() {
        let mut client: HydrusClient = HydrusClient::new("http://127.0.0.1:51251/");
        client.set_api_key("7ab7accf6cf12b2c6c30436cd8fe16361aee33679dbd90da279b5c22b33d622a");
        let _ = client.get_service_name("all my files").await.unwrap();
    }

    #[tokio::test]
    async fn test_service_key_info() {
        let mut client: HydrusClient = HydrusClient::new("http://127.0.0.1:51251/");
        client.set_api_key("7ab7accf6cf12b2c6c30436cd8fe16361aee33679dbd90da279b5c22b33d622a");
        let _ = client
            .get_service_key("616c6c206c6f63616c206d65646961")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_services() {
        let mut client: HydrusClient = HydrusClient::new("http://127.0.0.1:51251/");
        client.set_api_key("7ab7accf6cf12b2c6c30436cd8fe16361aee33679dbd90da279b5c22b33d622a");
        let res = client.get_services().await.unwrap();
        assert!(!res.is_empty())
    }
}
