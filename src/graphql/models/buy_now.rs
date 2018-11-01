use graphql::models::{CompaniesPackages, Coupon, Product};
use stq_types::Quantity;

#[derive(Deserialize, Debug, Clone)]
pub struct BuyNowCheckout {
    pub product: Product,
    pub quantity: Quantity,
    pub coupon: Option<Coupon>,
    pub company_package: Option<CompaniesPackages>, // TODO: replace AvailablePackageForUser
}
