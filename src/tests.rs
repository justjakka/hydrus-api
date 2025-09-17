use crate::client::HydrusClient;
use crate::{traits::*, types::*};

#[tokio::test]
async fn test_service_info() {
    let mut client: HydrusClient = HydrusClient::new("http://127.0.0.1:51251/");
    client.set_api_key("7ab7accf6cf12b2c6c30436cd8fe16361aee33679dbd90da279b5c22b33d622a");
    let res = client.get_service_name("all my files").await.unwrap();
    println!("{:?}", res);
}
