use async_trait::async_trait;
use maya_client_sdk::MayaClient;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Response,
};
use serde::Serialize;

pub mod payment_gateway {

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

    #[async_trait]
    pub trait PaymentGateway {
        async fn create_payment_token(
            &self,
            card_details: CardDetails,
        ) -> Result<Response, Box<dyn std::error::Error>>;
    }

    #[async_trait]
    impl PaymentGateway for MayaClient {
        async fn create_payment_token(
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
    }
}
