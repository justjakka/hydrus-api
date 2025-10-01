use crate::sync_lib::HydrusError;
use crate::sync_lib::traits::*;
use crate::types::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use ureq::{
    RequestBuilder,
    typestate::{WithBody, WithoutBody},
};

type Result<T> = std::result::Result<T, HydrusError>;

/// hydrus client
pub struct HydrusClient {
    client: ureq::Agent,
    apikey: Option<String>,
    sessionkey: Option<String>,
    url: String,
}

impl HydrusClient {
    /// create a new hydrus client object. requires a hydrus API endpoint url
    pub fn new(url: &str) -> HydrusClient {
        let mut url = url.to_string();
        if !url.ends_with('/') {
            url.push('/');
        }
        HydrusClient {
            client: ureq::Agent::new_with_defaults(),
            apikey: None,
            sessionkey: None,
            url,
        }
    }
    /// set an api key
    pub fn set_api_key(&mut self, key: &str) {
        self.apikey = Some(key.to_owned())
    }
    /// set a session key
    pub fn set_session_key(&mut self, key: &str) {
        self.sessionkey = Some(key.to_owned())
    }

    fn set_get_request_key(&self, url: &str) -> Result<RequestBuilder<WithoutBody>> {
        let request = self.client.get(url);
        if let Some(key) = &self.sessionkey {
            Ok(request.header("Hydrus-Client-API-Access-Key", key))
        } else if let Some(key) = &self.apikey {
            Ok(request.header("Hydrus-Client-API-Access-Key", key))
        } else {
            Err(HydrusError::KeyNotSupplied)
        }
    }

    fn set_post_request_key(&self, url: &str) -> Result<RequestBuilder<WithBody>> {
        let request = self.client.post(url);
        if let Some(key) = &self.sessionkey {
            Ok(request.header("Hydrus-Client-API-Access-Key", key))
        } else if let Some(key) = &self.apikey {
            Ok(request.header("Hydrus-Client-API-Access-Key", key))
        } else {
            Err(HydrusError::KeyNotSupplied)
        }
    }
}

#[derive(Deserialize, Debug)]
struct HydrusResponse<T> {
    #[serde(
        alias = "service",
        alias = "services",
        alias = "access_key",
        alias = "session_key"
    )]
    body: T,
}

impl AccessManagement for HydrusClient {
    fn request_new_permissions(
        &self,
        name: &str,
        permissions: &[HydrusPermissions],
    ) -> Result<String> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("request_new_permissions");

        let mut request = self.client.get(req_url);
        request = request.query("name", urlencoding::encode(name));

        if permissions.is_empty() {
            request = request.query("permit_everything", "true");
        } else {
            let json_string = serde_json::to_string(&permissions)?;
            request = request.query("basic_permissions", urlencoding::encode(&json_string));
        };

        Ok(request
            .call()?
            .body_mut()
            .read_json::<HydrusResponse<String>>()?
            .body)
    }

    fn get_session_key(&self) -> Result<String> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("session_key");

        let mut request = self.client.get(req_url);

        if let Some(key) = &self.apikey {
            request = request.header("Hydrus-Client-API-Access-Key", key);
        }

        Ok(request
            .call()?
            .body_mut()
            .read_json::<HydrusResponse<String>>()?
            .body)
    }

    fn verify_access_key(&self, key: &str) -> Result<KeyInfo> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("verify_access_key");

        Ok(self
            .client
            .get(req_url)
            .header("Hydrus-Client-API-Access-Key", key)
            .call()?
            .body_mut()
            .read_json::<KeyInfo>()?)
    }

    fn get_service_name(&self, name: &str) -> Result<Service> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("get_service");
        let mut request = self.set_get_request_key(&req_url)?;
        request = request.query("service_name", urlencoding::encode(name));

        Ok(request
            .call()?
            .body_mut()
            .read_json::<HydrusResponse<Service>>()?
            .body)
    }

    fn get_service_key(&self, key: &str) -> Result<Service> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("get_service");
        let mut request = self.set_get_request_key(&req_url)?;
        request = request.query("service_key", urlencoding::encode(key));

        Ok(request
            .call()?
            .body_mut()
            .read_json::<HydrusResponse<Service>>()?
            .body)
    }

    fn get_services(&self) -> Result<HashMap<String, Service>> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("get_services");

        let mut services = self
            .set_get_request_key(&req_url)?
            .call()?
            .body_mut()
            .read_json::<HydrusResponse<HashMap<String, Service>>>()?
            .body;

        for (key, service) in services.iter_mut() {
            service.service_key = key.to_string();
        }

        Ok(services)
    }
}

impl ImportingAndDeletingFiles for HydrusClient {
    fn add_file_via_path(
        &self,
        path: PathBuf,
        delete: Option<bool>,
        domains: Option<FileDomain>,
    ) -> Result<AddFileResponse> {
        let mut form = AddFileRequest {
            path: path.to_owned(),
            delete_after_successful_import: delete,
            ..Default::default()
        };

        if let Some(file_domain) = domains {
            match file_domain {
                FileDomain::FileServiceKey(key) => form.file_service_key = Some(key.to_string()),
                FileDomain::FileServiceKeys(keys) => form.file_service_keys = Some(keys.to_owned()),
                FileDomain::DeletedFileServiceKey(key) => {
                    form.deleted_file_service_key = Some(key.to_string())
                }
                FileDomain::DeletedFileServiceKeys(keys) => {
                    form.deleted_file_service_keys = Some(keys.to_owned())
                }
            }
        }

        let mut req_url = self.url.to_owned();
        req_url.push_str("add_files/add_file");

        Ok(self
            .set_post_request_key(&req_url)?
            .send_json(&form)?
            .body_mut()
            .read_json::<AddFileResponse>()?)
    }

    fn add_file_via_file(&self, file: PathBuf) -> Result<AddFileResponse> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("add_files/add_file");

        let file = File::open(&file)?;

        Ok(self
            .set_post_request_key(&req_url)?
            .header("Content-Type", "application/octet-stream")
            .send(&file)?
            .body_mut()
            .read_json::<AddFileResponse>()?)
    }

    fn delete_files(
        &self,
        file: HydrusFile,
        domain: Option<FileDomain>,
        reason: Option<String>,
    ) -> Result<()> {
        let mut form = FileRequest {
            file,
            reason,
            ..Default::default()
        };

        if let Some(file_domain) = domain {
            match file_domain {
                FileDomain::FileServiceKey(key) => form.file_service_key = Some(key.to_string()),
                FileDomain::FileServiceKeys(keys) => form.file_service_keys = Some(keys.to_owned()),
                FileDomain::DeletedFileServiceKey(key) => {
                    form.deleted_file_service_key = Some(key.to_string())
                }
                FileDomain::DeletedFileServiceKeys(keys) => {
                    form.deleted_file_service_keys = Some(keys.to_owned())
                }
            }
        }

        let mut req_url = self.url.to_owned();
        req_url.push_str("add_files/delete_files");

        let _ = self.set_post_request_key(&req_url)?.send_json(&form)?;

        Ok(())
    }

    fn undelete_files(&self, file: HydrusFile, domain: Option<FileDomain>) -> Result<()> {
        let mut form = FileRequest {
            file,
            ..Default::default()
        };

        if let Some(file_domain) = domain {
            match file_domain {
                FileDomain::FileServiceKey(key) => form.file_service_key = Some(key.to_string()),
                FileDomain::FileServiceKeys(keys) => form.file_service_keys = Some(keys.to_owned()),
                FileDomain::DeletedFileServiceKey(key) => {
                    form.deleted_file_service_key = Some(key.to_string())
                }
                FileDomain::DeletedFileServiceKeys(keys) => {
                    form.deleted_file_service_keys = Some(keys.to_owned())
                }
            }
        }

        let mut req_url = self.url.to_owned();
        req_url.push_str("add_files/undelete_files");

        let _ = self.set_post_request_key(&req_url)?.send_json(&form)?;

        Ok(())
    }

    fn clear_file_deletion_records(&self, file: HydrusFile) -> Result<()> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("add_files/clear_file_deletion_record");
        let _ = self.set_post_request_key(&req_url)?.send_json(&file)?;

        Ok(())
    }

    fn migrate_files(&self, file: HydrusFile, domain: FileDomain) -> Result<()> {
        let mut form = FileRequest {
            file,
            ..Default::default()
        };

        match domain {
            FileDomain::FileServiceKey(key) => form.file_service_key = Some(key.to_string()),
            FileDomain::FileServiceKeys(keys) => form.file_service_keys = Some(keys.to_owned()),
            FileDomain::DeletedFileServiceKey(key) => {
                form.deleted_file_service_key = Some(key.to_string())
            }
            FileDomain::DeletedFileServiceKeys(keys) => {
                form.deleted_file_service_keys = Some(keys.to_owned())
            }
        }

        let mut req_url = self.url.to_owned();
        req_url.push_str("add_files/migrate_files");
        let _ = self.set_post_request_key(&req_url)?.send_json(&form)?;

        Ok(())
    }

    fn archive_files(&self, file: HydrusFile) -> Result<()> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("add_files/archive_files");

        let form = FileRequest {
            file,
            ..Default::default()
        };

        let _ = self.set_post_request_key(&req_url)?.send_json(&form)?;

        Ok(())
    }

    fn unarchive_files(&self, file: HydrusFile) -> Result<()> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("add_files/unarchive_files");

        let form = FileRequest {
            file,
            ..Default::default()
        };

        let _ = self.set_post_request_key(&req_url)?.send_json(&form)?;

        Ok(())
    }

    fn generate_hashes_for_path(&self, file: PathBuf) -> Result<HashResponse> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("add_files/generate_hashes");

        let form = AddFileRequest {
            path: file.to_owned(),
            ..Default::default()
        };

        Ok(self
            .set_post_request_key(&req_url)?
            .send_json(&form)?
            .body_mut()
            .read_json::<HashResponse>()?)
    }

    fn generate_hashes_for_file(&self, file: PathBuf) -> Result<HashResponse> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("add_files/generate_hashes");

        let file = File::open(file)?;

        Ok(self
            .set_post_request_key(&req_url)?
            .header("Content-Type", "application/octet-stream")
            .send(&file)?
            .body_mut()
            .read_json::<HashResponse>()?)
    }
}

impl ImportingAndEditingUrls for HydrusClient {
    fn get_url_files(
        &self,
        url: &str,
        doublecheck_file_system: Option<bool>,
    ) -> Result<FilesUrlResponse> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("add_urls/get_url_files");

        let mut request = self.set_get_request_key(&req_url)?;

        request = request.query("url", url);

        if let Some(doublecheck) = doublecheck_file_system {
            request = request.query(
                "doublecheck_file_system",
                if doublecheck { "true" } else { "false" },
            );
        }

        Ok(request.call()?.body_mut().read_json::<FilesUrlResponse>()?)
    }
}
