use musli::{Allocator, Decode, Decoder, Encode, Encoder};
use std::collections::HashMap;
use strum_macros::FromRepr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HydrusError {
    #[error("failed to connect to Hydrus")]
    NetworkError(ureq::Error),
    #[error("failed to encode/decode data")]
    DeserializeError(musli::json::Error),
    #[error("api or session key needed")]
    KeyNotSupplied,
}

impl From<musli::json::Error> for HydrusError {
    fn from(value: musli::json::Error) -> Self {
        HydrusError::DeserializeError(value)
    }
}

impl From<ureq::Error> for HydrusError {
    fn from(value: ureq::Error) -> Self {
        HydrusError::NetworkError(value)
    }
}

#[derive(PartialEq, Debug, Clone, FromRepr)]
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

impl<M> Encode<M> for HydrusPermissions {
    type Encode = Self;

    #[inline]
    fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
    where
        E: Encoder<Mode = M>,
    {
        encoder.encode(self.clone() as u8)
    }

    #[inline]
    fn as_encode(&self) -> &Self::Encode {
        self
    }
}

impl<'de, M, A> Decode<'de, M, A> for HydrusPermissions
where
    A: Allocator,
{
    #[inline]
    fn decode<D>(decoder: D) -> Result<Self, D::Error>
    where
        D: Decoder<'de>,
    {
        Ok(HydrusPermissions::from_repr(decoder.decode()?).unwrap())
    }
}

#[derive(Decode, Debug)]
pub struct AccessKey {
    pub access_key: String,
}

#[derive(Decode, Debug)]
pub struct SessionKey {
    pub session_key: String,
}

#[derive(Decode, Debug)]
pub struct KeyInfo {
    pub name: String,
    pub permits_everything: bool,
    pub basic_permissions: Vec<HydrusPermissions>,
    pub human_permissions: String,
}

#[derive(PartialEq, Debug, Clone, FromRepr)]
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

impl<M> Encode<M> for ServiceType {
    type Encode = Self;

    #[inline]
    fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
    where
        E: Encoder<Mode = M>,
    {
        encoder.encode(self.clone() as u8)
    }

    #[inline]
    fn as_encode(&self) -> &Self::Encode {
        self
    }
}

impl<'de, M, A> Decode<'de, M, A> for ServiceType
where
    A: Allocator,
{
    #[inline]
    fn decode<D>(decoder: D) -> Result<Self, D::Error>
    where
        D: Decoder<'de>,
    {
        Ok(ServiceType::from_repr(decoder.decode()?).unwrap())
    }
}

#[derive(Decode, Debug)]
pub struct Service {
    pub name: String,
    #[musli(default)]
    pub service_key: String,
    pub r#type: ServiceType,
    pub type_pretty: String,
    #[musli(default)]
    pub star_shape: Option<String>,
    #[musli(default)]
    pub min_stars: Option<u8>,
    #[musli(default)]
    pub max_stars: Option<u8>,
}

#[derive(Decode, Debug)]
pub struct ServiceResponse {
    services: HashMap<String, Service>,
}

impl ServiceResponse {
    pub fn service_vec(mut self) -> Vec<Service> {
        let mut res = Vec::new();
        for (key, mut service) in self.services.drain() {
            service.service_key = key;
            res.push(service);
        }
        res
    }
}

pub enum FileDomain {
    FileServiceKey(String),
    FileServiceKeys(Vec<String>),
    DeletedFileServiceKey(String),
    DeletedFileServiceKeys(Vec<String>),
}

#[derive(Encode, Debug, Default)]
pub struct AddFileRequest {
    pub path: String,
    #[musli(skip)]
    pub delete_after_successful_import: Option<bool>,
    #[musli(skip)]
    pub file_service_key: Option<String>,
    #[musli(skip)]
    pub file_service_keys: Option<Vec<String>>,
    #[musli(skip)]
    pub deleted_file_service_key: Option<String>,
    #[musli(skip)]
    pub deleted_file_service_keys: Option<Vec<String>>,
}

#[derive(PartialEq, Debug, Clone, FromRepr)]
#[repr(u8)]
pub enum FileAddStatus {
    SuccessfulImport = 1,
    AlreadyInDatabase,
    PreviouslyDeleted,
    FailedToImport,
    FileVetoed = 7,
}

impl<M> Encode<M> for FileAddStatus {
    type Encode = Self;

    #[inline]
    fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
    where
        E: Encoder<Mode = M>,
    {
        encoder.encode(self.clone() as u8)
    }

    #[inline]
    fn as_encode(&self) -> &Self::Encode {
        self
    }
}

impl<'de, M, A> Decode<'de, M, A> for FileAddStatus
where
    A: Allocator,
{
    #[inline]
    fn decode<D>(decoder: D) -> Result<Self, D::Error>
    where
        D: Decoder<'de>,
    {
        Ok(FileAddStatus::from_repr(decoder.decode()?).unwrap())
    }
}

#[derive(Decode)]
pub struct FileAddResponse {
    pub status: FileAddStatus,
    pub hash: String,
    pub note: String,
}

#[derive(Encode, Debug, Default)]
pub struct DeleteFileRequest {
    pub path: String,
    #[musli(skip)]
    pub file_service_key: Option<String>,
    #[musli(skip)]
    pub file_service_keys: Option<Vec<String>>,
    #[musli(skip)]
    pub deleted_file_service_key: Option<String>,
    #[musli(skip)]
    pub deleted_file_service_keys: Option<Vec<String>>,
    #[musli(skip)]
    pub reason: Option<String>,
}
