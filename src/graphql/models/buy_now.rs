use graphql::models::{Coupon, Product};
use stq_types::Quantity;

#[derive(Deserialize, Debug, Clone)]
pub struct BuyNowCheckout {
    pub product: Product,
    pub quantity: Quantity,
    pub coupon: Option<Coupon>,
}
