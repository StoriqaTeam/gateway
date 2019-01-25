pub mod fee_id;
pub use self::fee_id::FeeId;

use stq_static_resources::Currency;
use stq_types::{stripe::ChargeId, OrderId};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Fee {
    pub id: FeeId,
    pub order_id: OrderId,
    pub amount: f64,
    pub status: FeeStatus,
    pub currency: Currency,
    pub charge_id: Option<ChargeId>,
}

#[derive(GraphQLEnum, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FeeStatus {
    NotPaid,
    Paid,
    Fail,
}
