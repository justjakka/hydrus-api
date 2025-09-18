use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use thiserror::Error;

/// various errors
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

/// hydrus permissions
#[derive(PartialEq, Debug, Clone, Serialize_repr, Deserialize_repr)]
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

/// hydrus key information struct
#[derive(Deserialize, Debug)]
pub struct KeyInfo {
    pub name: String,
    pub permits_everything: bool,
    pub basic_permissions: Vec<HydrusPermissions>,
    pub human_permissions: String,
}

/// hydrus service type struct
#[derive(PartialEq, Debug, Clone, Serialize_repr, Deserialize_repr)]
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

/// hydrus service struct
#[derive(Deserialize, Debug)]
pub struct Service {
    pub name: String,
    #[serde(default)]
    pub service_key: String,
    pub r#type: ServiceType,
    pub type_pretty: String,
    #[serde(default)]
    pub star_shape: Option<String>,
    #[serde(default)]
    pub min_stars: Option<u8>,
    #[serde(default)]
    pub max_stars: Option<u8>,
}

/// hydrus file domains
pub enum FileDomain {
    FileServiceKey(String),
    FileServiceKeys(Vec<String>),
    DeletedFileServiceKey(String),
    DeletedFileServiceKeys(Vec<String>),
}

/// payload for importing a file via providing a local path
#[derive(Serialize, Debug, Default)]
pub struct AddFileRequest {
    pub path: PathBuf,
    #[serde(skip)]
    pub delete_after_successful_import: Option<bool>,
    #[serde(skip)]
    pub file_service_key: Option<String>,
    #[serde(skip)]
    pub file_service_keys: Option<Vec<String>>,
    #[serde(skip)]
    pub deleted_file_service_key: Option<String>,
    #[serde(skip)]
    pub deleted_file_service_keys: Option<Vec<String>>,
}
/// file importing status
#[derive(PartialEq, Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AddFileStatus {
    SuccessfulImport = 1,
    AlreadyInDatabase,
    PreviouslyDeleted,
    FailedToImport,
    FileVetoed = 7,
}
/// file importing api response
#[derive(Deserialize)]
pub struct AddFileResponse {
    pub status: AddFileStatus,
    pub hash: String,
    pub note: String,
}
/// payload for deleting a file
#[derive(Serialize, Debug, Default)]
pub struct DeleteFileRequest {
    pub path: String,
    #[serde(skip)]
    pub file_service_key: Option<String>,
    #[serde(skip)]
    pub file_service_keys: Option<Vec<String>>,
    #[serde(skip)]
    pub deleted_file_service_key: Option<String>,
    #[serde(skip)]
    pub deleted_file_service_keys: Option<Vec<String>>,
    #[serde(skip)]
    pub reason: Option<String>,
}
