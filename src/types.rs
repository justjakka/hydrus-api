use musli::{Allocator, Decode, Decoder, Encode, Encoder};
use strum_macros::FromRepr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HydrusError {
    #[error("failed to connect to Hydrus")]
    NetworkError(ureq::Error),
    #[error("failed to deserialize data")]
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
        if let Some(val) = HydrusPermissions::from_repr(decoder.decode()?) {
            return Ok(val);
        } else {
            return Ok(Self::Null);
        }
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
        if let Some(val) = ServiceType::from_repr(decoder.decode()?) {
            return Ok(val);
        } else {
            return Ok(Self::Null);
        }
    }
}

#[derive(Decode)]
pub struct Service {
    pub name: String,
    #[musli(default)]
    pub service_key: String,
    pub servicetype: ServiceType,
    pub type_pretty: String,
    #[musli(default)]
    pub star_shape: String,
    #[musli(default)]
    pub min_stars: u8,
    #[musli(default)]
    pub max_stars: u8,
}
