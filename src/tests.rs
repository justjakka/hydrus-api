use crate::client::HydrusClient;
use crate::types::*;

#[test]
fn correct_hydruspermissions_url_encode() {
    let perms: [HydrusPermissions; 3] = [
        HydrusPermissions::ImportAndEditURLs,
        HydrusPermissions::ImportAndEditFiles,
        HydrusPermissions::SeeLocalPaths,
    ];
    let json_string = musli::json::to_string(&perms).unwrap();
    let encoded = urlencoding::encode(&json_string);

    assert_eq!(encoded, "%5B0%2C1%2C13%5D")
}

#[test]
fn correct_hydruspermissions_decode() {
    let perms: [HydrusPermissions; 3] = [
        HydrusPermissions::ImportAndEditURLs,
        HydrusPermissions::ImportAndEditFiles,
        HydrusPermissions::SeeLocalPaths,
    ];
    let json_string = musli::json::to_string(&perms).unwrap();

    let res: [HydrusPermissions; 3] = musli::json::from_str(&json_string).unwrap();
    assert_eq!(res, perms)
}

#[test]
fn correct_hydrusservice_decode() {
    let input: [ServiceType; 3] = [
        ServiceType::AllLocalFiles,
        ServiceType::ClientAPI,
        ServiceType::Trash,
    ];

    let json_string = musli::json::to_string(&input).unwrap();

    let res: [ServiceType; 3] = musli::json::from_str(&json_string).unwrap();
    assert_eq!(res, input)
}

#[test]
fn correct_hydrusservices_decode() {
    let mut client: HydrusClient = HydrusClient::new("http://127.0.0.1:51251/");
    client.set_api_key("7ab7accf6cf12b2c6c30436cd8fe16361aee33679dbd90da279b5c22b33d622a");
    let test = client.get_services().unwrap();
    assert!(!test.is_empty())
}
