use maya_client_sdk::MayaClient;
use maya_payment_vault_sdk::{CardDetails, PaymentGateway};

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
    async fn create_payment() {}
}
