use juniper::{FieldError, FieldResult, ID as GraphqlID};
use stq_static_resources::Currency;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Payout: Context as "Payout" |&self| {
    description: "Payout info"

    interfaces: [&Node]

    field id() -> GraphqlID as "Unique id" {
        self.id.to_string().into()
    }

    field gross_amount() -> f64 as "Gross payout amount without fees" {
        self.gross_amount.to_string().parse::<f64>().unwrap_or(0.0)
    }

    field net_amount() -> f64 as "Net payout amount with fees subtracted" {
        self.net_amount.to_string().parse::<f64>().unwrap_or(0.0)
    }

    field currency() -> Currency as "Currency of the payout" {
        match &self.target {
            PayoutTarget::CryptoWallet(target) => target.currency,
        }
    }

    field wallet_address() -> Option<String> as "Target wallet address" {
        match &self.target {
            PayoutTarget::CryptoWallet(target) => Some(target.wallet_address.clone()),
        }
    }

    field blockchain_fee() -> Option<f64> as "Blockchain fee for the transaction" {
        match &self.target {
            PayoutTarget::CryptoWallet(target) => Some(target.blockchain_fee.to_string().parse::<f64>().unwrap_or(0.0)),
        }
    }

    field initiated_at() -> String as "Payout initiation time" {
        match &self.status {
            PayoutStatus::Processing { initiated_at } => initiated_at.format("%+").to_string(),
            PayoutStatus::Completed { initiated_at, .. } => initiated_at.format("%+").to_string(),
        }
    }

    field completed_at() -> Option<String> as "Payout completion time" {
        match &self.status {
            PayoutStatus::Processing { .. } => None,
            PayoutStatus::Completed { completed_at, .. } => Some(completed_at.format("%+").to_string()),
        }
    }

    field order_ids() -> Vec<String> as "IDs of the billing orders liked to this payout" {
        self.order_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>()
    }
});

graphql_object!(PayoutCalculation: Context as "PayoutCalculation" |&self| {
    description: "Payout calculation"

    field order_ids() -> Vec<String> as "IDs of the billing orders included in the calculation" {
        self.order_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>()
    }

    field currency() -> Currency as "Currency of the payout calculation" {
        self.currency
    }

    field gross_amount() -> f64 as "Gross amount of the payout calculation" {
        self.gross_amount.to_string().parse::<f64>().unwrap_or(0.0)
    }

    field blockchain_fee_options() -> &[BlockchainFeeOption] as "Available blockchain fee options to select" {
        &self.blockchain_fee_options
    }
});

graphql_object!(BlockchainFeeOption: Context as "BlockchainFeeOption" |&self| {
    field value() -> String as "Blockchain fee value" {
        self.value.to_string()
    }

    field estimated_time_seconds() -> i32 as "Estimated time of blockchain transaction confirmation in seconds" {
        self.estimated_time_seconds
    }
});

graphql_object!(PayoutsByStoreId: Context as "PayoutsByStoreId" |&self| {
    field payouts() -> Vec<Payout> as "Payouts of the store" {
        self.payouts.iter().cloned().map(|p| p.payout).collect()
    }
});

graphql_object!(Balances: Context as "StoreBalance" |&self| {
    description: "Store billing balance"

    field stq() -> f64 as "STQ Balance" {
        self.currencies.get(&Currency::STQ)
        .map(|amount| amount.to_string().parse::<f64>().unwrap_or(0.0))
        .unwrap_or(0.0)
    }

    field btc(&executor) -> f64 as "BTC Balance" {
        self.currencies.get(&Currency::BTC)
        .map(|amount| amount.to_string().parse::<f64>().unwrap_or(0.0))
        .unwrap_or(0.0)
    }

    field eth(&executor) -> f64 as "ETH Balance" {
        self.currencies.get(&Currency::ETH)
        .map(|amount| amount.to_string().parse::<f64>().unwrap_or(0.0))
        .unwrap_or(0.0)
    }

    field eur(&executor) -> f64 as "EUR Balance" {
        self.currencies.get(&Currency::EUR)
        .map(|amount| amount.to_string().parse::<f64>().unwrap_or(0.0))
        .unwrap_or(0.0)
    }

});

pub fn run_pay_out_crypto_to_seller_mutation(context: &Context, input: PayOutCryptoToSellerInput) -> FieldResult<Payout> {
    let payload = input.try_into_payload().map_err(|e| match e {
        PayOutCryptoInputConversionError::InvalidOrderIdFormat => FieldError::new(
            "Invalid input",
            graphql_value!({ "code": 300, "details": { "Invalid order ID format" }}),
        ),
        PayOutCryptoInputConversionError::InvalidBlockchainFeeFormat => FieldError::new(
            "Invalid input",
            graphql_value!({ "code": 300, "details": { "Invalid blockchain fee format" }}),
        ),
    })?;

    context.get_billing_microservice().pay_out_to_seller(payload)
}
