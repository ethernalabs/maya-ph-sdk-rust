use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Card {
    number: String,
    expYear: String,
    expMonth: String,
    cvv: String,
}

#[tokio::main]
pub async fn create_payment_token(
    base_url: String,
    pan: String,
    expYear: String,
    expMonth: String,
    cvv: String,
) -> Result<reqwest::Response, reqwest::Error> {
    
  let client = reqwest::Client::new();
  let path = format!("{}{}", &base_url, "/payments/v1/payment-tokens");
  let card = Card {
        number: pan,
        expYear: expYear,
        expMonth: expMonth,
        cvv: cvv,
  };

  let res = client.post(&path).json(&card).send().await?;

  Ok(res)
}
