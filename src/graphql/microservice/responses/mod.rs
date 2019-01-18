use stripe::Card as StripeCard;

use stq_types::UserId;

use graphql::models::stripe::customer_id::CustomerId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomerResponse {
    pub id: CustomerId,
    pub user_id: UserId,
    pub email: Option<String>,
    pub cards: Vec<StripeCard>,
}
