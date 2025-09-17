use std::{collections::HashMap, path::PathBuf};

use crate::types::*;
use async_trait::async_trait;

type Result<T> = std::result::Result<T, HydrusError>;

#[async_trait]
pub trait AccessManagement {
    async fn request_new_permissions(
        &self,
        name: &str,
        permissions: &[HydrusPermissions],
    ) -> Result<String>;

    async fn get_session_key(&self) -> Result<String>;

    async fn verify_access_key(&self, key: &str) -> Result<KeyInfo>;

    async fn get_service_name(&self, name: &str) -> Result<Service>;

    async fn get_service_key(&self, key: &str) -> Result<Service>;

    async fn get_services(&self) -> Result<HashMap<String, Service>>;
}

#[async_trait]
pub trait ImportingAndDeletingFiles {
    async fn add_file_via_path(
        &self,
        path: &str,
        delete: Option<bool>,
        domains: Option<FileDomain>,
    ) -> Result<FileAddResponse>;

    async fn add_file_via_file(&self, file: PathBuf) -> Result<FileAddResponse>;
}
