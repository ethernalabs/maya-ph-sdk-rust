use reqwest::Error;
use serde::{Deserialize, Serialize};


mod payment_gateway {
  use super::*;

  #[derive(Debug, Serialize, Deserialize)]
  pub struct Card {
      number: String,
      expYear: String,
      expMonth: String,
      cvv: String,
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
