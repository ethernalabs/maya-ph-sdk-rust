use serde::{Deserialize, Serialize};


pub mod payment_gateway {
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
      
    let client = reqwest::Client::new();
    let path = format!("{}{}", &base_url, "/payments/v1/payment-tokens");

  
    let res = client.post(&path).json(&card).send().await?;
  
    Ok(res)
  }  
}
