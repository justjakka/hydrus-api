use musli::{Decode, Encode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HydrusError {
    #[error("failed to connect to Hydrus")]
    NetworkError(ureq::Error),
    #[error("failed to deserialize data")]
    DeserializeError(musli::Error),
}

impl Into

#[derive(Decode, Encode, PartialEq, Debug)]
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

#[derive(Decode)]
struct AccessKey {
    access_key: String,
}

#[derive(Decode)]
struct SessionKey {
    session_key: String,
}

#[derive(Decode)]
pub struct KeyInfo {
    name: String,
    permits_everything: bool,
    basic_permissions: Vec<HydrusPermissions>,
    human_permissions: String,
}

#[derive(Decode, Encode, PartialEq, Debug)]
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

#[derive(Decode)]
pub struct Service {
    name: String,
    #[musli(default)]
    service_key: String,
    servicetype: ServiceType,
    type_pretty: String,
}
