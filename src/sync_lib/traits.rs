use std::{collections::HashMap, path::PathBuf};

use crate::sync_lib::HydrusError;
use crate::types::*;

type Result<T> = std::result::Result<T, HydrusError>;

/// Trait for accessing and managing keys and services.
pub trait AccessManagement {
    /// Register a new external program with the client. This requires the 'add from api request' mini-dialog under services->review services to be open, otherwise it will 403.
    fn request_new_permissions(
        &self,
        name: &str,
        permissions: &[HydrusPermissions],
    ) -> Result<String>;
    /// Get a new session key.
    fn get_session_key(&self) -> Result<String>;
    /// Check your access key is valid.
    fn verify_access_key(&self, key: &str) -> Result<KeyInfo>;
    /// Ask the client about a specific service by providing its name.
    fn get_service_name(&self, name: &str) -> Result<Service>;
    /// Ask the client about a specific service by providing its key.
    fn get_service_key(&self, key: &str) -> Result<Service>;
    /// Ask the client about its services.
    fn get_services(&self) -> Result<HashMap<String, Service>>;
}

/// Trait for importing and deleting files.
pub trait ImportingAndDeletingFiles {
    /// Tell the client to import a file by providing a local (hydrus-local) file path.
    fn add_file_via_path(
        &self,
        path: PathBuf,
        delete: Option<bool>,
        domains: Option<FileDomain>,
    ) -> Result<AddFileResponse>;
    /// Tell the client to import a file by sending the file.
    fn add_file_via_file(&self, file: PathBuf) -> Result<AddFileResponse>;
    /// Tell the client to send files to the trash.
    fn delete_files(
        &self,
        file: HydrusFile,
        domain: Option<FileDomain>,
        reason: Option<String>,
    ) -> Result<()>;
    /// Tell the client to restore files that were previously deleted to their old file service(s).
    fn undelete_files(&self, file: HydrusFile, domain: Option<FileDomain>) -> Result<()>;
    /// Tell the client to forget that it once deleted files.
    fn clear_file_deletion_records(&self, file: HydrusFile) -> Result<()>;
    /// Copy files from one local file domain to another.
    fn migrate_files(&self, file: HydrusFile, domain: FileDomain) -> Result<()>;
    /// Tell the client to archive inboxed files.
    fn archive_files(&self, file: HydrusFile) -> Result<()>;
    /// Tell the client re-inbox archived files.
    fn unarchive_files(&self, file: HydrusFile) -> Result<()>;
    /// Generate hashes for an arbitrary file by providing a local path to the file.
    fn generate_hashes_for_path(&self, file: PathBuf) -> Result<HashResponse>;
    /// Generate hashes for an arbitrary file by sending the file.
    fn generate_hashes_for_file(&self, file: PathBuf) -> Result<HashResponse>;
}

pub trait ImportingAndEditingUrls {
    fn get_url_files(
        &self,
        url: &str,
        doublecheck_file_system: Option<bool>,
    ) -> Result<FilesUrlResponse>;
}
