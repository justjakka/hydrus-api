use crate::client_async::HydrusClient;
use crate::{traits_async::*, types::*};

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
