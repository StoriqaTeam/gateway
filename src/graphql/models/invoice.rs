use std::time::SystemTime;

use stq_static_resources::{Currency, OrderState};
use stq_types::{
    stripe::{ChargeId, PaymentIntentId},
    InvoiceId, ProductPrice,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Invoice {
    pub invoice_id: InvoiceId,
    pub transactions: Vec<Transaction>,
    pub amount: ProductPrice,
    pub currency: Currency,
    pub price_reserved: SystemTime,
    pub state: OrderState,
    pub wallet: Option<String>,
    pub amount_captured: ProductPrice,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub amount_captured: ProductPrice,
}

#[derive(GraphQLObject, Clone, Debug, Serialize, Deserialize)]
#[graphql(description = "Balance")]
pub struct MerchantBalance {
    #[graphql(description = "amount")]
    pub amount: f64,
    #[graphql(description = "currency")]
    pub currency: String,
    #[graphql(description = "status")]
    pub status: MerchantBalanceStatus,
}

#[derive(GraphQLEnum, Deserialize, Serialize, Debug, Clone, PartialEq, Copy)]
#[graphql(name = "MerchantBalanceStatus", description = "Balance Status")]
#[serde(rename_all = "lowercase")]
pub enum MerchantBalanceStatus {
    Active,
    Blocked,
}

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
