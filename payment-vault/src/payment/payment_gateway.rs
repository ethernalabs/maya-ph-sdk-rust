use super::*;

#[async_trait]
pub trait PaymentGateway {
    /// Returns a response that contains the paymentTokenId to be use for further transaction
    /// to the API endpoint else an std::error:Error
    async fn create_payment_token(
        &self,
        card_details: CardDetails,
    ) -> Result<Response, Box<dyn std::error::Error>>;

    /// Returns a response that contains the details of the payment transaction
    async fn create_payment(&self) -> Result<Response, Box<dyn std::error::Error>>;
}
#[allow(non_snake_case)]
impl ShippingAddress {
    pub fn new(
        line1: Option<String>,
        line2: Option<String>,
        city: Option<String>,
        state: Option<String>,
        zipCode: Option<String>,
        countryCode: Option<String>,
        firstName: String,
        middleName: Option<String>,
        lastName: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        shippingType: ShippingType,
    ) -> Self {
        Self {
            line1: line1,
            line2: line2,
            city: city,
            state: state,
            zipCode: zipCode,
            countryCode: countryCode,
            firstName: firstName,
            lastName: lastName,
            middleName: middleName,
            phone: phone,
            email: email,
            shippingType: shippingType,
        }
    }
}
#[allow(non_snake_case)]
impl BillingAddress {
    pub fn new(
        line1: Option<String>,
        line2: Option<String>,
        city: Option<String>,
        state: Option<String>,
        zipCode: Option<String>,
        countryCode: Option<String>,
    ) -> Self {
        Self {
            line1: line1,
            line2: line2,
            city: city,
            state: state,
            zipCode: zipCode,
            countryCode: countryCode,
        }
    }
}

/// Implement the PaymentGateway trait to MayaClient
#[async_trait]
impl PaymentGateway for MayaClient {
    /// Returns a Result response containing the Response or a std::error::Error when
    /// request is made to the API endpoint of payment-token
    ///
    /// It takes a parameter card_details with a type of CardDetails struct
    /// which then will be sent to the API endpoint of the payment token. The auth_header and
    /// url_domain are taken from self and used for generating the full endpoint url and
    /// authorization header needed to authenticate to the end point
    ///
    /// ```rust,no_run
    /// use maya_client_sdk::MayaClient;
    /// use maya_payment_vault_sdk::{CardDetails, PaymentGateway};
    ///
    ///
    /// #[tokio::test]
    /// async fn create_payment_token() {
    ///    let card_details = CardDetails {
    ///        number: NUMBER.to_string(),
    ///         expYear: EXP_YEAR.to_string(),
    ///         expMonth: EXP_MONTH.to_string(),
    ///         cvc: CVC.to_string(),
    ///     };
    ///
    ///     let maya_client = MayaClient::new(
    ///         env!("ACCESS_TOKEN").to_string(),
    ///         env!("SECRET_TOKEN").to_string(),
    ///         None,
    ///     );
    ///
    ///     let res = maya_client
    ///         .create_payment_token(card_details)
    ///         .await
    ///         .unwrap();
    ///
    ///     let status_code = res.status();
    ///
    ///     assert_eq!(status_code, 200);
    /// }
    ///
    ///
    /// ```
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

    async fn create_payment(&self) -> Result<Response, Box<dyn std::error::Error>> {
        todo!();
    }
}
