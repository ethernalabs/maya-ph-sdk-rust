use maya_client_sdk::MayaClient;
use maya_payment_vault_sdk::payment::payment_gateway::*;
use maya_payment_vault_sdk::payment::*;

#[cfg(test)]
mod test_payment_token {

    use super::*;

    const NUMBER: &str = env!("NUMBER");
    const EXP_YEAR: &str = env!("EXP_YEAR");
    const EXP_MONTH: &str = env!("EXP_MONTH");
    const CVC: &str = env!("CVC");

    #[tokio::test]
    async fn create_payment_token() {
        let card_details = CardDetails {
            number: NUMBER.to_string(),
            expYear: EXP_YEAR.to_string(),
            expMonth: EXP_MONTH.to_string(),
            cvc: CVC.to_string(),
        };

        let maya_client = MayaClient::new(
            env!("ACCESS_TOKEN").to_string(),
            env!("SECRET_TOKEN").to_string(),
            None,
        );

        let res = maya_client
            .create_payment_token(card_details)
            .await
            .unwrap();

        let status_code = res.status();

        assert_eq!(status_code, 200);
    }

    #[tokio::test]
    async fn create_payment() {
        let billing_address = BillingAddress {
            line1: "Street 123".to_string(),
            line2: "Brgy. Poblacion".to_string(),
            city: "Mandaluyong".to_string(),
            state: "Philippines".to_string(),
            zipCode: "2313".to_string(),
            countryCode: "PH".to_string(),
        };
        let shipping_address: ShippingAddress = ShippingAddress {
            line1: "Street 123".to_string(),
            line2: "Brgy. Poblacion".to_string(),
            city: "Mandaluyong".to_string(),
            state: "Philippines".to_string(),
            zipCode: "2313".to_string(),
            countryCode: "PH".to_string(),
            firstName: "John".to_string(),
            lastName: "Doe".to_string(),
            middleName: "".to_string(),
            email: "john.doe@example.com".to_string(),
            phone: "+63(2)1234567890".to_string(),
            shippingType: ShippingType::ST,
        };

        let contact: ContactDetails = ContactDetails {
            phone: "+63(2)1234567890".to_string(),
            email: "sample@google.com".to_string(),
        };

        let redirect_url: RedirectUrl = RedirectUrl {
            success: "https://www.merchantsite.com/success".to_string(),
            failure: "https://www.merchantsite.com/failure".to_string(),
            cancel: "https://www.merchantsite.com/cancel".to_string(),
        };

        let total_amount: TotalAmount = TotalAmount {
            amount: "100".to_string(),
            currency: "PHP".to_string(),
        };

        let buyer_user: Buyer = Buyer {
            billingAddress: billing_address,
            shippingAddress: shipping_address,
            firstName: "John".to_string(),
            lastName: "Doe".to_string(),
            middleName: "".to_string(),
            birthday: "".to_string(),
            customerSince: "".to_string(),
            contact: contact,
            sex: Sex::M,
        };

        let card_details = CardDetails {
            number: NUMBER.to_string(),
            expYear: EXP_YEAR.to_string(),
            expMonth: EXP_MONTH.to_string(),
            cvc: CVC.to_string(),
        };

        let maya_client = MayaClient::new(
            env!("ACCESS_TOKEN").to_string(),
            env!("SECRET_TOKEN").to_string(),
            None,
        );

        let payment_token = maya_client
            .create_payment_token(card_details)
            .await
            .unwrap()
            .json::<PaymentToken>()
            .await
            .unwrap();

        let payment = Payment {
            paymentTokenId: payment_token.paymentTokenId,
            totalAmount: total_amount,
            buyer: Some(buyer_user),
            redirectUrl: Some(redirect_url),
            metadata: None,
            requestReferenceNumber: Some("332211".to_string()),
        };
        let create_payment = maya_client.create_payment(payment).await.unwrap();
        println!("Created payment {:?}", create_payment);

        // assert_eq!(create_payment.json(), 200);
    }
}
