use std::fs::File;

use ureq::RequestBuilder;

use crate::types::*;

type Result<T> = std::result::Result<T, HydrusError>;

pub struct HydrusClient {
    apikey: Option<String>,
    sessionkey: Option<String>,
    url: String,
}

impl HydrusClient {
    pub fn new(url: &str) -> HydrusClient {
        HydrusClient {
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

    pub fn request_new_permissions(
        &self,
        name: &str,
        permissions: &[HydrusPermissions],
    ) -> Result<String> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("request_new_permissions?name=");
        req_url.push_str(&urlencoding::encode(name));

        if permissions.is_empty() {
            req_url.push_str("&permit_everything=true");
        } else {
            req_url.push_str("&basic_permissions=");
            let json_string = musli::json::to_string(&permissions)?;
            req_url.push_str(&urlencoding::encode(&json_string));
        };

        let response = ureq::get(req_url).call()?.body_mut().read_to_vec()?;

        let key: AccessKey = musli::json::decode(response.as_slice())?;

        Ok(key.access_key)
    }

    pub fn get_session_key(&self) -> Result<String> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("session_key");

        let mut request = ureq::get(req_url);

        if let Some(key) = &self.apikey {
            request = request.header("Hydrus-Client-API-Access-Key", key);
        }

        let response = request.call()?.body_mut().read_to_vec()?;

        let key: SessionKey = musli::json::decode(response.as_slice())?;

        Ok(key.session_key)
    }

    pub fn verify_access_key(&self, key: &str) -> Result<KeyInfo> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("verify_access_key");
        let response = ureq::get(req_url)
            .header("Hydrus-Client-API-Access-Key", key)
            .call()?
            .body_mut()
            .read_to_vec()?;

        let data: KeyInfo = musli::json::decode(response.as_slice())?;

        Ok(data)
    }

    fn set_get_api_key(
        &self,
        request: RequestBuilder<ureq::typestate::WithoutBody>,
    ) -> Result<RequestBuilder<ureq::typestate::WithoutBody>> {
        if let Some(key) = &self.sessionkey {
            Ok(request.header("Hydrus-Client-API-Access-Key", key))
        } else if let Some(key) = &self.apikey {
            Ok(request.header("Hydrus-Client-API-Access-Key", key))
        } else {
            Err(HydrusError::KeyNotSupplied)
        }
    }

    fn set_post_api_key(
        &self,
        request: RequestBuilder<ureq::typestate::WithBody>,
    ) -> Result<RequestBuilder<ureq::typestate::WithBody>> {
        if let Some(key) = &self.sessionkey {
            Ok(request.header("Hydrus-Client-API-Access-Key", key))
        } else if let Some(key) = &self.apikey {
            Ok(request.header("Hydrus-Client-API-Access-Key", key))
        } else {
            Err(HydrusError::KeyNotSupplied)
        }
    }

    pub fn get_service_name(&self, name: &str) -> Result<Service> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("get_service?service_name=");
        req_url.push_str(&urlencoding::encode(name));

        let response = self
            .set_get_api_key(ureq::get(req_url))?
            .call()?
            .body_mut()
            .read_to_vec()?;

        let service: Service = musli::json::decode(response.as_slice())?;
        Ok(service)
    }

    pub fn get_service_key(&self, key: &str) -> Result<Service> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("get_service?service_key=");
        req_url.push_str(&urlencoding::encode(key));

        let response = self
            .set_get_api_key(ureq::get(req_url))?
            .call()?
            .body_mut()
            .read_to_vec()?;

        let service: Service = musli::json::decode(response.as_slice())?;
        Ok(service)
    }

    pub fn get_services(&self) -> Result<Vec<Service>> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("get_services");

        let response = self
            .set_get_api_key(ureq::get(req_url))?
            .call()?
            .body_mut()
            .read_to_vec()?;

        let services: ServiceResponse = musli::json::decode(response.as_slice())?;

        Ok(services.service_vec())
    }

    pub fn add_file_via_path(
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

        let data = musli::json::to_vec(&form)?;

        let mut req_url = self.url.to_owned();
        req_url.push_str("add_files/add_file");

        let response = self
            .set_post_api_key(ureq::post(req_url))?
            .content_type("application/json")
            .send(data)?
            .body_mut()
            .read_to_vec()?;

        let status: FileAddResponse = musli::json::decode(response.as_slice())?;
        Ok(status)
    }

    pub fn add_file_via_file(&self, file: &File) -> Result<FileAddResponse> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("add_files/add_file");

        let response = self
            .set_post_api_key(ureq::post(req_url))?
            .content_type("application/octet-stream")
            .send(file)?
            .body_mut()
            .read_to_vec()?;

        let status: FileAddResponse = musli::json::decode(response.as_slice())?;
        Ok(status)
    }
}
