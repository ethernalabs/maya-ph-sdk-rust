use serde::{Deserialize, Serialize};
use maya_client_sdk::MayaClient;

pub mod payment_gateway {
  use reqwest::header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE};

  use super::*;

  
  #[derive(Debug, Serialize, Deserialize)]
  #[allow(non_snake_case)]
  pub struct Card {
      pub number: String,
      pub expYear: String,
      pub expMonth: String,
      pub cvv: String,
  }
  
  #[tokio::main]
  pub async fn create_payment_token(
      base_url: String,
      card: Card,
  ) -> Result<reqwest::Response, reqwest::Error> {
    
    let token: String = MayaClient::new("", "").token;
    let client = reqwest::Client::new();
    let path: String = format!("{}{}", &base_url, "/payments/v1/payment-tokens");

    
    let res = client.post(&path)
                              .json(&card)
                              .header(CONTENT_TYPE, "application/json")
                              .header(AUTHORIZATION, &token)
                              .header(ACCEPT, "application/json")
                              .send().await?;

    println!("{:#?}", res);
    Ok(res)
  }  
}
