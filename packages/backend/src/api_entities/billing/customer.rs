use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use stripe::{Address, PaymentMethod};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct APIBillingCustomer {
    pub entity_name: Option<String>,
    pub email: Option<String>,
    pub address: Option<APIBillingCustomerAddress>,
    pub default_payment_method: Option<APIBillingCustomerPaymentMethod>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct APIBillingCustomerAddress {
    city: Option<String>,
    country: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl From<Address> for APIBillingCustomerAddress {
    fn from(value: Address) -> Self {
        Self {
            city: value.city,
            country: value.country,
            line1: value.line1,
            line2: value.line2,
            postal_code: value.postal_code,
            state: value.state,
        }
    }
}

impl From<APIBillingCustomerAddress> for Address {
    fn from(value: APIBillingCustomerAddress) -> Self {
        Self {
            city: value.city,
            country: value.country,
            line1: value.line1,
            line2: value.line2,
            postal_code: value.postal_code,
            state: value.state,
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct APIBillingCustomerPaymentMethodCard {
    last4: String,
    brand: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum APIBillingCustomerPaymentMethod {
    Card(APIBillingCustomerPaymentMethodCard),
    Other(String),
}

impl From<PaymentMethod> for APIBillingCustomerPaymentMethod {
    fn from(value: PaymentMethod) -> Self {
        if let Some(card) = value.card {
            Self::Card(APIBillingCustomerPaymentMethodCard {
                last4: card.last4,
                brand: card.brand,
            })
        } else {
            Self::Other(value.type_.to_string())
        }
    }
}
