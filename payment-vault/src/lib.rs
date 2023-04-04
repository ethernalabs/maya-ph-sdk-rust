use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client, Response,
};
use serde::Serialize;

pub mod payment_gateway {
    const CUSTOM_ENGINE: engine::GeneralPurpose =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

    use super::*;

    #[derive(Debug, Serialize)]
    #[allow(non_snake_case)]
    pub struct CardDetails {
        pub number: String,
        pub expYear: String,
        pub expMonth: String,
        pub cvc: String,
    }

    #[derive(Debug, Serialize)]
    pub struct Card {
        pub card: CardDetails,
    }

    pub struct MayaClient {
        client: Client,
        url_domain: String,
        auth_header: String,
    }

    impl MayaClient {
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

        fn generate_auth_header_value(username: &str, password: &str) -> String {
            let mut token = String::new();
            let token_format = format!("{}:{}", username, password);

            CUSTOM_ENGINE.encode_string(&token_format.as_bytes(), &mut token);

            return token;
        }

        pub async fn create_payment_token(
            &self,
            card_details: CardDetails,
        ) -> Result<Response, Box<dyn std::error::Error>> {
            let url = format!("https://{}/payments/v1/payment-tokens", self.url_domain);
            let auth = format!("Basic {}", &self.auth_header);
            let request_body = Card { card: card_details };

            let response = self
                .client
                .post(&url)
                .header(AUTHORIZATION, auth)
                .header(ACCEPT, "application/json")
                .header(CONTENT_TYPE, "application/json")
                .json(&request_body)
                .send()
                .await?
                .error_for_status()?;

            Ok(response)
        }

        pub async fn create_payment() -> () {}
    }
}
