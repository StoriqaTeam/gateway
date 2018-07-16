use chrono::prelude::*;
use stq_static_resources::OrderState;
use stq_types::{CurrencyId, InvoiceId, ProductPrice};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Invoice {
    pub invoice_id: InvoiceId,
    pub transaction_id: Option<String>,
    pub transaction_captured_amount: Option<ProductPrice>,
    pub amount: ProductPrice,
    pub currency_id: CurrencyId,
    pub price_reserved: DateTime<Utc>,
    pub state: OrderState,
    pub wallet: String,
}
