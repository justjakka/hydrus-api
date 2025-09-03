use std::error::Error;

use serde::Deserialize;
use serde_json::json;
use serde_repr::{Deserialize_repr, Serialize_repr};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct HydrusClient {
    apikey: Option<String>,
    url: String,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum HydrusPermissions {
    ImportAndEditURLs = 0,
    ImportAndEditFiles,
    EditFileTags,
    SearchAndFetchFiles,
    ManagePages,
    ManageCookiesAndHeaders,
    ManageDatabase,
    EditFileNotes,
    EditFileRelationships,
    EditFileRatings,
    ManagePopups,
    EditFileTimes,
    CommitPending,
    SeeLocalPaths,
}

#[derive(Deserialize)]
struct SessionKey {
    session_key: String,
}

impl HydrusClient {
    pub fn new(url: String) -> HydrusClient {
        return HydrusClient {
            apikey: None,
            url: url,
        };
    }

    pub fn set_api_key(&mut self, key: String) {
        self.apikey = Some(key)
    }

    pub fn request_new_permissions(
        &self,
        name: String,
        permissions: &[HydrusPermissions],
    ) -> Result<String> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("/request_new_permissions?name=");
        req_url.push_str(&name);

        if permissions.len() == 0 {
            req_url.push_str("&permit_everything=true");
        } else {
            req_url.push_str("&basic_permissions=");
            let json_string = serde_json::to_string(&json!(permissions))?;
            let encoded = urlencoding::encode(&json_string);
            req_url.push_str(&encoded);
        };

        let respose = ureq::get(req_url)
            .call()?
            .body_mut()
            .read_json::<SessionKey>()?;

        return Ok(respose.session_key);
    }
}
