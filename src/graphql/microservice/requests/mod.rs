use stq_types::{BaseProductId, OrderId, ProductId};

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
