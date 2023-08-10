use super::*;
pub mod payment_gateway;

/// Details for creating payment token
#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct CardDetails {
    pub number: String,
    pub expYear: String,
    pub expMonth: String,
    pub cvc: String,
}

#[derive(Debug, Serialize)]
struct Card {
    pub card: CardDetails,
}

#[derive(Debug, Serialize)]
pub enum ShippingType {
    ST,
    SD,
}

#[derive(Debug, Serialize)]
pub struct ContactDetails {
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum Sex {
    M,
    F,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Buyer<T = Sex> {
    pub firstName: String,
    pub middleName: Option<String>,
    pub lastName: String,
    pub birthday: String,
    // Birthday in ISO 8601 / RFC 3339 full-date format.
    pub customerSince: String,
    // Date in ISO 8601 / RFC 3339 full-date format
    pub sex: T,
    pub contact: ContactDetails,
    pub billingAddress: BillingAddress,
    pub shippingAddress: ShippingAddress,
}

#[derive(Debug, Serialize)]
pub struct RedirectUrl {
    pub success: Option<String>,
    pub failure: Option<String>,
    pub cancel: Option<String>,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct ShippingAddress<T = ShippingType> {
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zipCode: Option<String>,
    // ISO 3166 alpha-2 notion of the country
    pub countryCode: String,
    pub firstName: String,
    pub lastName: String,
    pub middleName: String,
    pub phone: String,
    pub email: String,
    shippingType: T,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct BillingAddress {
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zipCode: Option<String>,
    // ISO 3166 alpha-2 notion of the country
    pub countryCode: String,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct TotalAmount {
    pub amount: String,
    // ISO 4217 Alpha-3 currency code
    pub currency: String,
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Payment {
    paymentTokenId: String,
    pub totalAmount: TotalAmount,
    pub buyer: Buyer,
}
