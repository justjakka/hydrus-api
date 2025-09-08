use core::panic;
use musli::{Allocator, Decode, Decoder, Encode, Encoder};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HydrusError {
    #[error("failed to connect to Hydrus")]
    NetworkError(ureq::Error),
    #[error("failed to deserialize data")]
    DeserializeError(musli::json::Error),
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

#[derive(PartialEq, Debug, Clone)]
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

impl From<u8> for HydrusPermissions {
    fn from(value: u8) -> Self {
        match value {
            0 => HydrusPermissions::ImportAndEditURLs,
            1 => HydrusPermissions::ImportAndEditFiles,
            2 => HydrusPermissions::EditFileTags,
            3 => HydrusPermissions::SearchAndFetchFiles,
            4 => HydrusPermissions::ManagePages,
            5 => HydrusPermissions::ManageCookiesAndHeaders,
            6 => HydrusPermissions::ManageDatabase,
            7 => HydrusPermissions::EditFileNotes,
            8 => HydrusPermissions::EditFileRelationships,
            9 => HydrusPermissions::EditFileRatings,
            10 => HydrusPermissions::ManagePopups,
            11 => HydrusPermissions::EditFileTimes,
            12 => HydrusPermissions::CommitPending,
            13 => HydrusPermissions::SeeLocalPaths,
            _ => panic!("incorrect permission id"),
        }
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
        let val: u8 = decoder.decode()?;
        Ok(val.into())
    }
}

#[derive(Decode)]
pub struct AccessKey {
    pub access_key: String,
}

#[derive(Decode)]
pub struct SessionKey {
    pub session_key: String,
}

#[derive(Decode)]
pub struct KeyInfo {
    pub name: String,
    pub permits_everything: bool,
    pub basic_permissions: Vec<HydrusPermissions>,
    pub human_permissions: String,
}

#[derive(PartialEq, Debug, Clone)]
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

impl From<u8> for ServiceType {
    fn from(value: u8) -> Self {
        match value {
            0 => ServiceType::TagRepository,
            1 => ServiceType::FileRepository,
            2 => ServiceType::LocalFileDomain,
            5 => ServiceType::LocalTagDomain,
            6 => ServiceType::NumericalRating,
            7 => ServiceType::BoolRating,
            10 => ServiceType::AllKnownTags,
            11 => ServiceType::AllKnownFiles,
            12 => ServiceType::LocalBooru,
            13 => ServiceType::IPFS,
            14 => ServiceType::Trash,
            15 => ServiceType::AllLocalFiles,
            17 => ServiceType::FileNotes,
            18 => ServiceType::ClientAPI,
            19 => ServiceType::DeletedFromAnywhere,
            20 => ServiceType::LocalUpdates,
            21 => ServiceType::AllMyFiles,
            22 => ServiceType::IncDecRating,
            99 => ServiceType::ServerAdmin,
            _ => panic!("incorrect service id"),
        }
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
        Ok(decoder.decode()?)
    }
}

#[derive(Decode)]
pub struct Service {
    pub name: String,
    #[musli(default)]
    pub service_key: String,
    pub servicetype: ServiceType,
    pub type_pretty: String,
}
