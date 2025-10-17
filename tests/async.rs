use hydrus_api::async_lib::{client::*, traits::*};
use hydrus_api::types::*;
use std::path::PathBuf;

fn init_client() -> HydrusClient {
    let keyfile = PathBuf::from("secrets");
    let key = std::fs::read_to_string(keyfile).unwrap().trim().to_string();
    let mut client: HydrusClient = HydrusClient::new("http://127.0.0.1:45869/");
    client.set_api_key(key);
    client
}

#[tokio::test]
async fn test_service_name_info() {
    let client = init_client();
    let _ = client.get_service_name("all my files").await.unwrap();
}

#[tokio::test]
async fn test_service_key_info() {
    let client = init_client();
    let _ = client
        .get_service_key("616c6c206c6f63616c206d65646961")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_services() {
    let client = init_client();
    let res = client.get_services().await.unwrap();
    assert!(!res.is_empty())
}

#[tokio::test]
async fn add_and_delete_file() {
    let client = init_client();
    let path = PathBuf::from("./assets/img.png").canonicalize().unwrap();
    let file = client
        .add_file_via_path(path.clone(), None, None)
        .await
        .unwrap();
    if client
        .add_file_via_path(path.clone(), None, None)
        .await
        .unwrap()
        .status
        != AddFileStatus::AlreadyInDatabase
    {
        panic!();
    }

    let trash_service = Some(FileDomain::FileServiceKey(
        client
            .get_service_name("all my files")
            .await
            .unwrap()
            .service_key,
    ));

    client
        .delete_files(HydrusFile::Hash(file.hash.clone()), None, None)
        .await
        .unwrap();
    client
        .delete_files(HydrusFile::Hash(file.hash.clone()), trash_service, None)
        .await
        .unwrap();

    client
        .clear_file_deletion_records(HydrusFile::Hash(file.hash))
        .await
        .unwrap();
    let file = client.add_file_via_file(path.clone()).await.unwrap();
    if client.add_file_via_file(path).await.unwrap().status != AddFileStatus::AlreadyInDatabase {
        panic!();
    }
    client
        .delete_files(HydrusFile::Hash(file.hash.clone()), None, None)
        .await
        .unwrap();
    client
        .delete_files(HydrusFile::Hash(file.hash.clone()), None, None)
        .await
        .unwrap();

    client
        .clear_file_deletion_records(HydrusFile::Hash(file.hash))
        .await
        .unwrap();
}
