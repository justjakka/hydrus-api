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
            let json_string = serde_json::to_string(&permissions)?;
            req_url.push_str(&urlencoding::encode(&json_string));
        };

        let key: AccessKey = ureq::get(req_url).call()?.body_mut().read_json()?;

        Ok(key.access_key)
    }

    pub fn get_session_key(&self) -> Result<String> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("session_key");

        let mut request = ureq::get(req_url);

        if let Some(key) = &self.apikey {
            request = request.header("Hydrus-Client-API-Access-Key", key);
        }

        let key: SessionKey = request.call()?.body_mut().read_json()?;

        Ok(key.session_key)
    }

    pub fn verify_access_key(&self, key: &str) -> Result<KeyInfo> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("verify_access_key");
        let keyinfo: KeyInfo = ureq::get(req_url)
            .header("Hydrus-Client-API-Access-Key", key)
            .call()?
            .body_mut()
            .read_json()?;

        Ok(keyinfo)
    }

    pub fn get_service_name(&self, name: &str) -> Result<Service> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("get_service?service_name=");
        req_url.push_str(&urlencoding::encode(name));
        let service: Service = if let Some(key) = &self.sessionkey {
            ureq::get(req_url)
                .header("Hydrus-Client-API-Access-Key", key)
                .call()?
                .body_mut()
                .read_json()?
        } else if let Some(key) = &self.apikey {
            ureq::get(req_url)
                .header("Hydrus-Client-API-Access-Key", key)
                .call()?
                .body_mut()
                .read_json()?
        } else {
            return Err(HydrusError::KeyNotSupplied);
        };

        Ok(service)
    }

    pub fn get_service_key(&self, name: &str) -> Result<Service> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("get_service?service_key=");
        req_url.push_str(&urlencoding::encode(name));
        let service: Service = if let Some(key) = &self.sessionkey {
            ureq::get(req_url)
                .header("Hydrus-Client-API-Access-Key", key)
                .call()?
                .body_mut()
                .read_json()?
        } else if let Some(key) = &self.apikey {
            ureq::get(req_url)
                .header("Hydrus-Client-API-Access-Key", key)
                .call()?
                .body_mut()
                .read_json()?
        } else {
            return Err(HydrusError::KeyNotSupplied);
        };

        Ok(service)
    }

    pub fn get_services(&self) -> Result<Vec<Service>> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("get_services");
        let response: ServiceResponse = if let Some(key) = &self.sessionkey {
            ureq::get(req_url)
                .header("Hydrus-Client-API-Access-Key", key)
                .call()?
                .body_mut()
                .read_json()?
        } else if let Some(key) = &self.apikey {
            ureq::get(req_url)
                .header("Hydrus-Client-API-Access-Key", key)
                .call()?
                .body_mut()
                .read_json()?
        } else {
            return Err(HydrusError::KeyNotSupplied);
        };
        println!("{:?}", response);
        Ok(vec![])
    }
}
