use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HydrusError {
    #[error("failed to connect to Hydrus")]
    NetworkError(ureq::Error),
    #[error("failed to deserialize data")]
    DeserializeError(serde_json::Error),
    #[error("api or session key needed")]
    KeyNotSupplied,
}

impl From<serde_json::Error> for HydrusError {
    fn from(value: serde_json::Error) -> Self {
        HydrusError::DeserializeError(value)
    }
}

impl From<ureq::Error> for HydrusError {
    fn from(value: ureq::Error) -> Self {
        HydrusError::NetworkError(value)
    }
}

#[derive(PartialEq, Debug, Clone, Deserialize_repr, Serialize_repr)]
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
    Null = 255,
}

#[derive(Deserialize, Debug)]
pub struct AccessKey {
    pub access_key: String,
}

#[derive(Deserialize, Debug)]
pub struct SessionKey {
    pub session_key: String,
}

#[derive(Deserialize, Debug)]
pub struct KeyInfo {
    pub name: String,
    pub permits_everything: bool,
    pub basic_permissions: Vec<HydrusPermissions>,
    pub human_permissions: String,
}

#[derive(PartialEq, Debug, Clone, Deserialize_repr, Serialize_repr)]
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
    Null = 255,
}

#[derive(Deserialize, Debug)]
pub struct Service {
    pub name: String,
    #[serde(default)]
    pub service_key: String,
    pub r#type: ServiceType,
    pub type_pretty: String,
    #[serde(default)]
    pub star_shape: String,
    #[serde(default)]
    pub min_stars: u8,
    #[serde(default)]
    pub max_stars: u8,
}

#[derive(Deserialize, Debug)]
pub struct ServiceResponse {
    pub services: HashMap<String, Service>,
    #[serde(flatten)]
    _extra: HashMap<String, serde_json::Value>,
}
