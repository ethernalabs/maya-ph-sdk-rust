//! #MayaClient
//!
//! MayaClient creates and returns the url_domain and auth_header that is needed for API
//! authentication allong side with a client that is based on reqwest::Client

use reqwest::Client;

use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

#[derive(Debug)]
pub struct MayaClient {
    pub client: Client,
    pub url_domain: String,
    pub auth_header: String,
}

const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

impl MayaClient {
    /// Creates a client from reqwest client and returns  client, auth_header and the url domain
    /// for the client
    /// ```
    /// use maya_client_sdk::MayaClient;
    ///
    /// let maya_client = MayaClient::new(
    ///   "username".to_string(),
    ///   "password".to_string(),
    ///   None
    /// );
    ///
    /// ````
    pub fn new(username: String, password: String, url_domain: Option<String>) -> Self {
        let auth_header = Self::generate_auth_header_value(&username, &password);
        let client = Client::new();

        let domain = match url_domain {
            Some(v) => v,
            None => "pg-sandbox.paymaya.com".to_string(),
        };
        MayaClient {
            client,
            auth_header,
            url_domain: domain,
        }
    }

    /// Generates and returns a base64 value by combining the username and password separated by a colon
    fn generate_auth_header_value(username: &str, password: &str) -> String {
        let mut token = String::new();
        let token_format = format!("{}:{}", username, password);

        CUSTOM_ENGINE.encode_string(token_format.as_bytes(), &mut token);

        token
    }
}
