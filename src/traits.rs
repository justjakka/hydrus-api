use std::{collections::HashMap, path::PathBuf};

use crate::types::*;
use async_trait::async_trait;

type Result<T> = std::result::Result<T, HydrusError>;

/// trait for accessing and managing keys and services
#[async_trait]
pub trait AccessManagement {
    /// request new api key
    async fn request_new_permissions(
        &self,
        name: &str,
        permissions: &[HydrusPermissions],
    ) -> Result<String>;
    /// request new session key
    async fn get_session_key(&self) -> Result<String>;
    /// verify session or api key
    async fn verify_access_key(&self, key: &str) -> Result<KeyInfo>;
    /// get service info by providing service name
    async fn get_service_name(&self, name: &str) -> Result<Service>;
    /// get service info by providing service key
    async fn get_service_key(&self, key: &str) -> Result<Service>;
    /// get all service info
    async fn get_services(&self) -> Result<HashMap<String, Service>>;
}

/// trait for importing and deleting files
#[async_trait]
pub trait ImportingAndDeletingFiles {
    /// import file into hydrus by providing a local (hydrus-local) file path
    async fn add_file_via_path(
        &self,
        path: PathBuf,
        delete: Option<bool>,
        domains: Option<FileDomain>,
    ) -> Result<AddFileResponse>;
    /// import file into hydrus by sending the file
    async fn add_file_via_file(&self, file: PathBuf) -> Result<AddFileResponse>;
}
