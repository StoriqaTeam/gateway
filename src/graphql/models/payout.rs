use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use stq_static_resources::Currency;
use stq_types::{OrderId, PayoutId, StoreId, UserId};
use uuid::Uuid;

use graphql::microservice::{CryptoPaymentDetails, PayOutToSellerPayload, PaymentDetails};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payout {
    pub id: PayoutId,
    pub gross_amount: BigDecimal,
    pub net_amount: BigDecimal,
    pub target: PayoutTarget,
    pub user_id: UserId,
    pub status: PayoutStatus,
    pub order_ids: Vec<OrderId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutCalculation {
    pub order_ids: Vec<OrderId>,
    pub currency: Currency,
    pub gross_amount: BigDecimal,
    pub blockchain_fee_options: Vec<BlockchainFeeOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutsByStoreId {
    pub store_id: StoreId,
    pub payouts: Vec<PayoutWithOrderId>,
    pub order_ids_without_payout: Vec<OrderId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutWithOrderId {
    pub order_id: OrderId,
    pub payout: Payout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainFeeOption {
    pub value: BigDecimal,
    pub estimated_time_seconds: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PayoutTarget {
    CryptoWallet(CryptoWalletPayoutTarget),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CryptoWalletPayoutTarget {
    pub currency: Currency,
    pub wallet_address: String,
    pub blockchain_fee: BigDecimal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PayoutStatus {
    Processing {
        initiated_at: NaiveDateTime,
    },
    Completed {
        initiated_at: NaiveDateTime,
        completed_at: NaiveDateTime,
    },
}

#[derive(GraphQLInputObject, Clone, Debug)]
#[graphql(description = "Payout calculation input object")]
pub struct CalculatePayoutInput {
    #[graphql(description = "Currency of the target wallet")]
    pub currency: Currency,
    #[graphql(description = "Address of the target wallet")]
    pub wallet_address: String,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "Pay out crypto to seller input object")]
pub struct PayOutCryptoToSellerInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "IDs of the orders to include in the payout. All orders must have the same currency")]
    pub order_ids: Vec<String>,
    #[graphql(description = "Currency of the payout. Must be the same as the currency of the orders. Must be a cryptocurrency")]
    pub wallet_currency: Currency,
    #[graphql(description = "Target blockchain wallet address")]
    pub wallet_address: String,
    #[graphql(description = "Blockchain fee amount that was selected by the user")]
    pub blockchain_fee: String,
}

#[derive(Debug, Clone, Fail)]
pub enum PayOutCryptoInputConversionError {
    #[fail(display = "invalid order ID format")]
    InvalidOrderIdFormat,
    #[fail(display = "invalid blockchain fee format")]
    InvalidBlockchainFeeFormat,
}

impl PayOutCryptoToSellerInput {
    pub fn try_into_payload(self) -> Result<PayOutToSellerPayload, PayOutCryptoInputConversionError> {
        let PayOutCryptoToSellerInput {
            client_mutation_id: _,
            order_ids,
            wallet_currency,
            wallet_address,
            blockchain_fee,
        } = self;

        let order_ids = order_ids
            .into_iter()
            .map(|order_id| order_id.parse::<Uuid>().map(OrderId).map_err(|_| ()))
            .collect::<Result<Vec<_>, ()>>()
            .map_err(|_| PayOutCryptoInputConversionError::InvalidOrderIdFormat)?;

        let blockchain_fee = blockchain_fee
            .parse::<BigDecimal>()
            .map_err(|_| PayOutCryptoInputConversionError::InvalidBlockchainFeeFormat)?;

        Ok(PayOutToSellerPayload {
            order_ids,
            payment_details: PaymentDetails::Crypto(CryptoPaymentDetails {
                wallet_currency: wallet_currency.to_string().to_ascii_lowercase(),
                wallet_address,
                blockchain_fee,
            }),
        })
    }
}
