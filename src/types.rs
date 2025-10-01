use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Hydrus service permissions object
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

/// Hydrus key information struct
#[derive(Deserialize, Debug)]
pub struct KeyInfo {
    pub name: String,
    pub permits_everything: bool,
    pub basic_permissions: Vec<HydrusPermissions>,
    pub human_permissions: String,
}

/// Hydrus service type object
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

/// Hydrus service struct
#[derive(Deserialize, Debug, Clone)]
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

/// Hydrus file domains
pub enum FileDomain {
    FileServiceKey(String),
    FileServiceKeys(Vec<String>),
    DeletedFileServiceKey(String),
    DeletedFileServiceKeys(Vec<String>),
}

/// Payload for importing a file via providing a local path
#[derive(Serialize, Debug, Default)]
pub struct AddFileRequest {
    pub path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_after_successful_import: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_service_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_service_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_file_service_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_file_service_keys: Option<Vec<String>>,
}
/// File importing status
#[derive(PartialEq, Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AddFileStatus {
    SuccessfulImport = 1,
    AlreadyInDatabase,
    PreviouslyDeleted,
    FailedToImport,
    FileVetoed = 7,
}
/// File importing api response
#[derive(Deserialize)]
pub struct AddFileResponse {
    pub status: AddFileStatus,
    pub hash: String,
    pub note: String,
}

/// Hydrus file object
#[derive(Debug, Clone, Serialize)]
pub enum HydrusFile {
    #[serde(rename(serialize = "file_id"))]
    FileId(String),
    #[serde(rename(serialize = "file_ids"))]
    FileIds(Vec<String>),
    #[serde(rename(serialize = "hash"))]
    Hash(String),
    #[serde(rename(serialize = "hashes"))]
    Hashes(Vec<String>),
}

impl Default for HydrusFile {
    fn default() -> Self {
        Self::FileId(String::from(""))
    }
}

/// Payload for various file-related requests
#[derive(Debug, Default)]
pub struct FileRequest {
    pub file: HydrusFile,
    pub delete_after_successful_import: Option<bool>,
    pub file_service_key: Option<String>,
    pub file_service_keys: Option<Vec<String>>,
    pub deleted_file_service_key: Option<String>,
    pub deleted_file_service_keys: Option<Vec<String>>,
    pub reason: Option<String>,
}

impl Serialize for FileRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(1))?;

        match &self.file {
            HydrusFile::FileId(id) => map.serialize_entry("file_id", &id)?,
            HydrusFile::FileIds(ids) => map.serialize_entry("file_ids", &ids)?,
            HydrusFile::Hash(hash) => map.serialize_entry("hash", &hash)?,
            HydrusFile::Hashes(hashes) => map.serialize_entry("hashes", &hashes)?,
        }

        if let Some(val) = &self.reason {
            map.serialize_entry("reason", &val)?;
        }

        if let Some(val) = &self.file_service_key {
            map.serialize_entry("file_service_key", &val)?;
        }

        if let Some(val) = &self.file_service_keys {
            map.serialize_entry("file_service_keys", &val)?;
        }

        if let Some(val) = &self.deleted_file_service_key {
            map.serialize_entry("deleted_file_service_key", &val)?;
        }

        if let Some(val) = &self.deleted_file_service_keys {
            map.serialize_entry("deleted_file_service_keys", &val)?;
        }

        map.end()
    }
}

#[derive(Debug, Deserialize)]
pub struct HashResponse {
    pub hash: String,
    #[serde(default)]
    pub perceptual_hashes: Option<Vec<String>>,
    #[serde(default)]
    pub pixel_hash: Option<String>,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum UrlStatus {
    NotInDatabase = 0,
    AlreadyInDatabase = 2,
    PreviouslyDeleted,
}

#[derive(Debug, Deserialize)]
pub struct UrlFileStatus {
    pub status: UrlStatus,
    pub hash: String,
    pub note: String,
}

#[derive(Debug, Deserialize)]
pub struct FilesUrlResponse {
    pub normalised_url: String,
    pub url_file_statuses: Vec<UrlFileStatus>,
}
