use hydrus_api_rs::sync_lib::client::HydrusClient;
use hydrus_api_rs::sync_lib::traits::*;
use std::path::PathBuf;

#[test]
fn test_service_name_info() {
    let keyfile = PathBuf::from("secrets");
    let key = std::fs::read_to_string(keyfile).unwrap().trim().to_string();
    let mut client: HydrusClient = HydrusClient::new("http://127.0.0.1:45869/");
    client.set_api_key(key);
    let _ = client.get_service_name("all my files").unwrap();
}

#[test]
fn test_service_key_info() {
    let keyfile = PathBuf::from("secrets");
    let mut client: HydrusClient = HydrusClient::new("http://127.0.0.1:45869/");
    let key = std::fs::read_to_string(keyfile).unwrap().trim().to_string();
    client.set_api_key(key);
    let _ = client
        .get_service_key("616c6c206c6f63616c206d65646961")
        .unwrap();
}

#[test]
fn test_get_services() {
    let mut client: HydrusClient = HydrusClient::new("http://127.0.0.1:45869/");
    let keyfile = PathBuf::from("secrets");
    let key = std::fs::read_to_string(keyfile).unwrap().trim().to_string();
    client.set_api_key(key);
    let res = client.get_services().unwrap();
    assert!(!res.is_empty())
}
