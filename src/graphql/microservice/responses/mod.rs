use bigdecimal::BigDecimal;
use stq_types::OrderId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutCalculation {
    pub order_ids: Vec<OrderId>,
    pub currency: String,
    pub gross_amount: BigDecimal,
    pub blockchain_fee_options: Vec<BlockchainFeeOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainFeeOption {
    pub value: BigDecimal,
    pub estimated_time_seconds: i32,
}
