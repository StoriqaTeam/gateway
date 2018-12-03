use graphql::models::{AvailablePackageForUser, Coupon, Product};
use stq_types::Quantity;

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "Calculate buy now input object")]
pub struct CalculateBuyNowInput {
    #[graphql(description = "Product raw id")]
    pub product_id: i32,
    #[graphql(description = "Quantity")]
    pub quantity: i32,
    #[graphql(description = "User country code")]
    pub user_country_code: String,
    #[graphql(description = "Coupon code")]
    pub coupon_code: Option<String>,
    #[graphql(description = "Raw shipping id")]
    pub shipping_id: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct BuyNowCheckout {
    pub user_country_code: Option<String>,
    pub product: Product,
    pub quantity: Quantity,
    pub coupon: Option<Coupon>,
    pub package: Option<AvailablePackageForUser>,
}
