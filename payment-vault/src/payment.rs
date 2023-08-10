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

#[derive(Debug)]
pub enum ShippingType {
    ST,
    SD,
}

pub struct ContactDetails {
    pub phone: Option<String>,
    pub email: Option<String>
}

pub enum Sex {
    "M",
    "F"
}

#[derive(Debug)]
pub struct Buyer<T = Sex> {
    pub firstName: String,
    pub middleName: Option<String>,
    pub lastName: String,
    pub birthday: String, /// Birthday in ISO 8601 / RFC 3339 full-date format.
    pub customerSince: String, /// Date in ISO 8601 / RFC 3339 full-date format 
    pub sex: T,
    pub contact: ContactDetails

}

#[derive(Debug)]
pub struct RedirectUrl {
    pub success: Option<String>,
    pub failure: Option<String>,
    pub cancel: Option<String>
}

#[derive(Debug, Serialize)]
#[allow(non_snake_case)]
pub struct Address<T = ShippingType> {
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zipCode: Option<String>,
    pub countryCode: String, /// ISO 3166 aplha-2 notion of the country
    pub firstName: Option<String>,
    pub middleName: Option<String>,
    pub lastName: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub shippingType: T,
}

