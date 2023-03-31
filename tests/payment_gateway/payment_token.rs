use maya_sdk_rust::payment_gateway::payment_token::payment_token;
#[cfg(test)]
mod test_payment_token {
    use super::*;

    const BASE_URL: &str = "https://pg-sandbox.paymaya.com";
    const NUMBER: &str = "5453010000064154";
    const EXP_YEAR: &str = "2025";
    const EXP_MONTH: &str = "12";
    const CVV: &str = "111";

    #[test]
    fn create_payment_token() {
        let result = payment_token(
            BASE_URL.to_string(),
            NUMBER.to_string(),
            EXP_YEAR.to_string(),
            EXP_MONTH.to_string(),
            CVV.to_string(),
        );
        let status = result.unwrap().status();
        assert_eq!(status, 200);
    }
}
