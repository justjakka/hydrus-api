use std::{collections::HashMap, path::PathBuf};

use crate::types::*;
use async_trait::async_trait;

type Result<T> = std::result::Result<T, HydrusError>;

/// Trait for accessing and managing keys and services.
#[async_trait]
pub trait AccessManagement {
    /// Register a new external program with the client. This requires the 'add from api request' mini-dialog under services->review services to be open, otherwise it will 403.
    async fn request_new_permissions(
        &self,
        name: &str,
        permissions: &[HydrusPermissions],
    ) -> Result<String>;
    /// Get a new session key.
    async fn get_session_key(&self) -> Result<String>;
    /// Check your access key is valid.
    async fn verify_access_key(&self, key: &str) -> Result<KeyInfo>;
    /// Ask the client about a specific service by providing its name.
    async fn get_service_name(&self, name: &str) -> Result<Service>;
    /// Ask the client about a specific service by providing its key.
    async fn get_service_key(&self, key: &str) -> Result<Service>;
    /// Ask the client about its services.
    async fn get_services(&self) -> Result<HashMap<String, Service>>;
}

/// Trait for importing and deleting files.
#[async_trait]
pub trait ImportingAndDeletingFiles {
    /// Tell the client to import a file by providing a local (hydrus-local) file path.
    async fn add_file_via_path(
        &self,
        path: PathBuf,
        delete: Option<bool>,
        domains: Option<FileDomain>,
    ) -> Result<AddFileResponse>;
    /// Tell the client to import a file by sending the file.
    async fn add_file_via_file(&self, file: PathBuf) -> Result<AddFileResponse>;
    /// Tell the client to send files to the trash.
    async fn delete_files(
        &self,
        file: HydrusFile,
        domain: Option<FileDomain>,
        reason: Option<String>,
    ) -> Result<()>;
    /// Tell the client to restore files that were previously deleted to their old file service(s).
    async fn undelete_files(&self, file: HydrusFile, domain: Option<FileDomain>) -> Result<()>;
    /// Tell the client to forget that it once deleted files.
    async fn clear_file_deletion_records(&self, file: HydrusFile) -> Result<()>;
    /// Copy files from one local file domain to another.
    async fn migrate_files(&self, file: HydrusFile, domain: FileDomain) -> Result<()>;
    /// Tell the client to archive inboxed files.
    async fn archive_files(&self, file: HydrusFile) -> Result<()>;
    /// Tell the client re-inbox archived files.
    async fn unarchive_files(&self, file: HydrusFile) -> Result<()>;
    /// Generate hashes for an arbitrary file by providing a local path to the file.
    async fn generate_hashes_for_path(&self, file: PathBuf) -> Result<HashResponse>;
    /// Generate hashes for an arbitrary file by sending the file.
    async fn generate_hashes_for_file(&self, file: PathBuf) -> Result<HashResponse>;
}

#[async_trait]
pub trait ImportingAndEditingUrls {
    async fn get_url_files(
        &self,
        url: &str,
        doublecheck_file_system: Option<bool>,
    ) -> Result<FilesUrlResponse>;
}
