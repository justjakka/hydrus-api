use serde::Deserialize;
use serde_json::json;
use serde_repr::{Deserialize_repr, Serialize_repr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HydrusError {
    #[error("failed to connect to Hydrus")]
    Unavaliable(ureq::Error),
}

type Result<T> = std::result::Result<T, HydrusError>;

pub struct HydrusClient {
    apikey: Option<String>,
    sessionkey: Option<String>,
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
struct AccessKey {
    access_key: String,
}

#[derive(Deserialize)]
struct SessionKey {
    session_key: String,
}

#[derive(Deserialize)]
pub struct KeyInfo {
    name: String,
    permits_everything: bool,
    basic_permissions: Vec<HydrusPermissions>,
    human_permissions: String,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ServiceType {
    TagRepository = 0,
    FileRepository,
    LocalFileDomain,
    LocalTagDomain = 5,
    NumericalRating,
    BoolRating,
    AllKnownTags = 10,
    AllKnownFiles,
    LocalBooru,
    IPFS,
    Trash,
    AllLocalFiles,
    FileNotes = 17,
    ClientAPI,
    DeletedFromAnywhere,
    LocalUpdates,
    AllMyFiles,
    IncDecRating,
    ServerAdmin = 99,
}

#[derive(Deserialize)]
pub struct Service {
    name: String,
    servicetype: ServiceType,
    type_pretty: String,
}

#[derive(Deserialize)]
pub struct GetService {
    name: String,
    service_key: String,
    servicetype: ServiceType,
    type_pretty: String,
}

impl HydrusClient {
    pub fn new(url: String) -> HydrusClient {
        HydrusClient {
            apikey: None,
            sessionkey: None,
            url,
        }
    }

    pub fn set_api_key(&mut self, key: String) {
        self.apikey = Some(key)
    }

    pub fn set_session_key(&mut self, key: String) {
        self.sessionkey = Some(key)
    }

    pub fn request_new_permissions(
        &self,
        name: String,
        permissions: &[HydrusPermissions],
    ) -> Result<String, ureq::Error> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("/request_new_permissions?name=");
        req_url.push_str(&name);

        if permissions.is_empty() {
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
            .read_json::<AccessKey>()?;

        Ok(respose.access_key)
    }

    pub fn get_session_key(&self) -> Result<String, ureq::Error> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("/session_key");

        let mut request = ureq::get(req_url);

        if let Some(key) = &self.apikey {
            request = request.header("Hydrus-Client-API-Access-Key", key);
        }

        let respose = request.call()?.body_mut().read_json::<SessionKey>()?;

        Ok(respose.session_key)
    }

    pub fn verify_access_key(&self, key: String) -> Result<KeyInfo, ureq::Error> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("/verify_access_key");
        ureq::get(req_url)
            .header("Hydrus-Client-API-Access-Key", key)
            .call()?
            .body_mut()
            .read_json::<KeyInfo>()
    }

    pub fn get_service_name(&self, name: String) -> Result<GetService, ureq::Error> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("/get_service?service_name=");
        req_url.push_str(&name);
    }
}
