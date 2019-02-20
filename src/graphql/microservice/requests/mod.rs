use bigdecimal::BigDecimal;
use stq_types::{BaseProductId, OrderId, ProductId, StoreId};

use graphql::models::customer_id::CustomerId;
use graphql::models::*;

#[derive(Serialize, Debug, Clone)]
pub struct NewCustomerWithSourceRequest {
    pub email: Option<String>,
    pub card_token: String,
}

impl From<CreateCustomerWithSourceInput> for NewCustomerWithSourceRequest {
    fn from(other: CreateCustomerWithSourceInput) -> Self {
        let CreateCustomerWithSourceInput { email, card_token, .. } = other;

        Self { email, card_token }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteCustomerRequest {
    pub customer_id: CustomerId,
}

#[derive(Debug, Clone, Serialize)]
pub struct FeesPayByOrdersRequest {
    pub order_ids: Vec<OrderId>,
}

impl From<DeleteCustomerInput> for DeleteCustomerRequest {
    fn from(other: DeleteCustomerInput) -> Self {
        Self {
            customer_id: CustomerId::new(other.id),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GetBaseProductsRequest {
    pub ids: Vec<BaseProductId>,
}

#[derive(Debug, Serialize)]
pub struct GetProductsRequest {
    pub ids: Vec<ProductId>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CalculatePayoutPayload {
    pub store_id: StoreId,
    pub currency: String,
    pub wallet_address: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PayOutToSellerPayload {
    pub order_ids: Vec<OrderId>,
    pub payment_details: PaymentDetails,
}

#[derive(Debug, Clone, Serialize)]
pub enum PaymentDetails {
    Crypto(CryptoPaymentDetails),
}

#[derive(Debug, Clone, Serialize)]
pub struct CryptoPaymentDetails {
    pub wallet_currency: String,
    pub wallet_address: String,
    pub blockchain_fee: BigDecimal,
}
