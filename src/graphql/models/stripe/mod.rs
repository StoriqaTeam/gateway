pub mod customer_id;

use stripe::{Card as StripeCard, CardBrand as StripeCardBrand};

use stq_static_resources::Currency;
use stq_types::{
    stripe::{ChargeId, PaymentIntentId},
    InvoiceId, UserId,
};

use graphql::microservice::responses::CustomerResponse;

use self::customer_id::CustomerId;

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Stripe Customer input.")]
pub struct CreateCustomerWithSourceInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Customer’s email address.")]
    pub email: Option<String>,
    #[graphql(description = "Credit card token for use Stripe API.")]
    pub card_token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentIntent {
    pub id: PaymentIntentId,
    pub invoice_id: InvoiceId,
    pub amount: f64,
    pub amount_received: f64,
    pub client_secret: Option<String>,
    pub currency: Currency,
    pub last_payment_error_message: Option<String>,
    pub receipt_email: Option<String>,
    pub charge_id: Option<ChargeId>,
    pub status: PaymentIntentStatus,
}

#[derive(GraphQLEnum, Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentStatus {
    RequiresSource,
    RequiresConfirmation,
    RequiresSourceAction,
    Processing,
    RequiresCapture,
    Canceled,
    Succeeded,
    #[serde(other)]
    Other,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Customer {
    pub id: CustomerId,
    pub user_id: UserId,
    pub email: Option<String>,
    pub cards: Vec<Card>,
}

impl From<CustomerResponse> for Customer {
    fn from(other: CustomerResponse) -> Self {
        let cards = other.cards.into_iter().map(|card| Card::from(card)).collect();

        Self {
            id: other.id,
            user_id: other.user_id,
            email: other.email,
            cards,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Card {
    pub id: String,
    pub brand: CardBrand,
    pub country: String,
    pub customer: Option<String>,
    pub exp_month: u32,
    pub exp_year: u32,
    pub last4: String,
    pub name: Option<String>,
}

impl From<StripeCard> for Card {
    fn from(other: StripeCard) -> Self {
        Self {
            id: other.id,
            brand: other.brand.into(),
            country: other.country,
            customer: other.customer,
            exp_month: other.exp_month,
            exp_year: other.exp_year,
            last4: other.last4,
            name: other.name,
        }
    }
}

#[derive(GraphQLEnum, Deserialize, Serialize, PartialEq, Debug, Clone, Eq)]
pub enum CardBrand {
    AmericanExpress,
    DinersClub,
    Discover,
    JCB,
    Visa,
    MasterCard,
    UnionPay,
    #[serde(other)]
    Unknown,
}

impl From<StripeCardBrand> for CardBrand {
    fn from(other: StripeCardBrand) -> Self {
        match other {
            StripeCardBrand::AmericanExpress => CardBrand::AmericanExpress,
            StripeCardBrand::DinersClub => CardBrand::DinersClub,
            StripeCardBrand::Discover => CardBrand::Discover,
            StripeCardBrand::JCB => CardBrand::JCB,
            StripeCardBrand::Visa => CardBrand::Visa,
            StripeCardBrand::MasterCard => CardBrand::MasterCard,
            StripeCardBrand::UnionPay => CardBrand::UnionPay,
            StripeCardBrand::Unknown => CardBrand::Unknown,
        }
    }
}
