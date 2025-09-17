use std::{collections::HashMap, path::PathBuf};

use async_trait::async_trait;
use reqwest::{Body, RequestBuilder};
use serde::Deserialize;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{traits::*, types::*};

type Result<T> = std::result::Result<T, HydrusError>;

pub struct HydrusClient {
    client: reqwest::Client,
    apikey: Option<String>,
    sessionkey: Option<String>,
    url: String,
}

impl HydrusClient {
    pub fn new(url: &str) -> HydrusClient {
        HydrusClient {
            client: reqwest::Client::new(),
            apikey: None,
            sessionkey: None,
            url: url.to_string(),
        }
    }

    pub fn set_api_key(&mut self, key: &str) {
        self.apikey = Some(key.to_owned())
    }

    pub fn set_session_key(&mut self, key: &str) {
        self.sessionkey = Some(key.to_owned())
    }

    fn set_get_request_key(&self, url: &str) -> Result<RequestBuilder> {
        let request = self.client.get(url);
        if let Some(key) = &self.sessionkey {
            Ok(request.header("Hydrus-Client-API-Access-Key", key))
        } else if let Some(key) = &self.apikey {
            Ok(request.header("Hydrus-Client-API-Access-Key", key))
        } else {
            Err(HydrusError::KeyNotSupplied)
        }
    }

    fn set_post_request_key(&self, url: &str) -> Result<RequestBuilder> {
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

#[async_trait]
impl AccessManagement for HydrusClient {
    async fn request_new_permissions(
        &self,
        name: &str,
        permissions: &[HydrusPermissions],
    ) -> Result<String> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("request_new_permissions?name=");

        req_url.push_str(name);

        if permissions.is_empty() {
            req_url.push_str("&permit_everything=true");
        } else {
            let json_string = &serde_json::to_string(&permissions)?;
            req_url.push_str("&basic_permissions=");
            req_url.push_str(&urlencoding::encode(json_string));
        };

        Ok(self
            .client
            .get(req_url)
            .send()
            .await?
            .json::<HydrusResponse<String>>()
            .await?
            .body)
    }

    async fn get_session_key(&self) -> Result<String> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("session_key");

        let mut request = self.client.get(req_url);

        if let Some(key) = &self.apikey {
            request = request.header("Hydrus-Client-API-Access-Key", key);
        }

        Ok(request
            .send()
            .await?
            .json::<HydrusResponse<String>>()
            .await?
            .body)
    }

    async fn verify_access_key(&self, key: &str) -> Result<KeyInfo> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("verify_access_key");

        Ok(self
            .client
            .get(req_url)
            .header("Hydrus-Client-API-Access-Key", key)
            .send()
            .await?
            .json::<KeyInfo>()
            .await?)
    }

    async fn get_service_name(&self, name: &str) -> Result<Service> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("get_service?service_name=");
        req_url.push_str(&urlencoding::encode(name));

        Ok(self
            .set_get_request_key(&req_url)?
            .send()
            .await?
            .json::<HydrusResponse<Service>>()
            .await?
            .body)
    }

    async fn get_service_key(&self, key: &str) -> Result<Service> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("get_service?service_key=");
        req_url.push_str(&urlencoding::encode(key));

        Ok(self
            .set_get_request_key(&req_url)?
            .send()
            .await?
            .json::<HydrusResponse<Service>>()
            .await?
            .body)
    }

    async fn get_services(&self) -> Result<HashMap<String, Service>> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("get_services");

        let mut services = self
            .set_get_request_key(&req_url)?
            .send()
            .await?
            .json::<HydrusResponse<HashMap<String, Service>>>()
            .await?
            .body;

        for (key, service) in services.iter_mut() {
            service.service_key = key.to_string();
        }

        Ok(services)
    }
}

#[async_trait]
impl ImportingAndDeletingFiles for HydrusClient {
    async fn add_file_via_path(
        &self,
        path: &str,
        delete: Option<bool>,
        domains: Option<FileDomain>,
    ) -> Result<FileAddResponse> {
        let mut form = AddFileRequest {
            path: path.to_owned(),
            delete_after_successful_import: delete,
            ..Default::default()
        };

        if let Some(file_domains) = domains {
            match file_domains {
                FileDomain::FileServiceKey(key) => form.file_service_key = Some(key),
                FileDomain::FileServiceKeys(keys) => form.file_service_keys = Some(keys),
                FileDomain::DeletedFileServiceKey(key) => form.deleted_file_service_key = Some(key),
                FileDomain::DeletedFileServiceKeys(keys) => {
                    form.deleted_file_service_keys = Some(keys)
                }
            }
        }

        let mut req_url = self.url.to_owned();
        req_url.push_str("add_files/add_file");

        Ok(self
            .set_post_request_key(&req_url)?
            .header("Content-Type", "application/json")
            .json(&form)
            .send()
            .await?
            .json::<FileAddResponse>()
            .await?)
    }

    async fn add_file_via_file(&self, file: PathBuf) -> Result<FileAddResponse> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("add_files/add_file");

        let file = tokio::fs::File::open(file).await?;

        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);

        Ok(self
            .set_post_request_key(&req_url)?
            .header("Content-Type", "application/octet-stream")
            .body(body)
            .send()
            .await?
            .json::<FileAddResponse>()
            .await?)
    }
}
