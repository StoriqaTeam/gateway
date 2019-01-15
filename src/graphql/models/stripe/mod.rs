use stq_static_resources::Currency;
use stq_types::{
    stripe::{ChargeId, PaymentIntentId},
    InvoiceId,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaymentIntent {
    pub id: PaymentIntentId,
    pub invoice_id: InvoiceId,
    pub amount: u64,
    pub amount_received: u64,
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

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Stripe Customer input.")]
pub struct CreateCustomerWithSourceInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Customer’s email address.")]
    pub email: String,
    #[graphql(description = "Credit card token for use Stripe API.")]
    pub card_token: String,
}
