use crate::types::*;

type Result<T> = std::result::Result<T, HydrusError>;

pub struct HydrusClient {
    apikey: Option<String>,
    sessionkey: Option<String>,
    url: String,
}

impl HydrusClient {
    pub fn new(url: String) -> HydrusClient {
        HydrusClient {
            apikey: None,
            sessionkey: None,
            url,
        }
    }

    pub fn set_api_key(&mut self, key: String) {
        self.apikey = Some(key)
    }

    pub fn set_session_key(&mut self, key: String) {
        self.sessionkey = Some(key)
    }

    pub fn request_new_permissions(
        &self,
        name: String,
        permissions: &[HydrusPermissions],
    ) -> Result<String> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("/request_new_permissions?name=");
        req_url.push_str(&name);

        if permissions.is_empty() {
            req_url.push_str("&permit_everything=true");
        } else {
            req_url.push_str("&basic_permissions=");
            let json_string = musli::json::to_string(&permissions).unwrap();
            req_url.push_str(&urlencoding::encode(&json_string));
        };

        let response = ureq::get(req_url).call();

        if let Err(error) = response {
            return Err(HydrusError::NetworkError(error));
        }

        let key = response.unwrap().body_mut().read_to_vec();

        if let Err(error) = key {
            return Err(HydrusError::DeserializeError(error));
        };

        let accesskey: String = musli::json::decode(key.unwrap().as_slice()).unwrap();

        Ok(key.unwrap().access_key)
    }

    pub fn get_session_key(&self) -> Result<String> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("/session_key");

        let mut request = ureq::get(req_url);

        if let Some(key) = &self.apikey {
            request = request.header("Hydrus-Client-API-Access-Key", key);
        }

        let response = request.call();

        if let Err(error) = response {
            return Err(HydrusError::NetworkError(error));
        }

        let key = response.unwrap().body_mut().read_json::<SessionKey>();

        if let Err(error) = key {
            return Err(HydrusError::DeserializeError(error));
        }

        Ok(key.unwrap().session_key)
    }

    pub fn verify_access_key(&self, key: String) -> Result<KeyInfo> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("/verify_access_key");
        let response = ureq::get(req_url)
            .header("Hydrus-Client-API-Access-Key", key)
            .call();

        if let Err(error) = response {
            return Err(HydrusError::NetworkError(error));
        }

        let data = response.unwrap().body_mut().read_json::<KeyInfo>();

        if let Err(error) = data {
            return Err(HydrusError::DeserializeError(error));
        }

        Ok(data.unwrap())
    }

    pub fn get_service_name(&self, name: String) -> Result<GetService> {
        let mut req_url = self.url.to_owned();
        req_url.push_str("/get_service?service_name=");
        req_url.push_str(&urlencoding::encode(&name));
        todo!()
    }
}
