use std::collections::HashMap;
use stq_static_resources::Currency;
use stq_types::{CurrencyExchangeId, ExchangeRate};

pub type ExchangeRates = HashMap<Currency, ExchangeRate>;
pub type CurrencyExchangeData = HashMap<Currency, ExchangeRates>;

#[derive(Clone, Debug, Deserialize)]
pub struct CurrencyExchangeInfo {
    pub id: CurrencyExchangeId,
    pub data: CurrencyExchangeData,
}

#[derive(GraphQLObject, Serialize, Deserialize, Clone, Debug, Default)]
pub struct CurrencyExchange {
    pub code: &'static str,
    pub rates: Vec<CurrencyExchangeValue>,
}

impl CurrencyExchange {
    pub fn from_data(v: CurrencyExchangeData) -> Vec<CurrencyExchange> {
        v.into_iter()
            .map(|(cur, rates)| CurrencyExchange {
                code: cur.code(),
                rates: CurrencyExchangeValue::from_data(rates),
            }).collect()
    }
}

#[derive(GraphQLObject, Serialize, Deserialize, Clone, Debug)]
pub struct CurrencyExchangeValue {
    pub code: &'static str,
    pub value: f64,
}

impl CurrencyExchangeValue {
    pub fn from_data(v: ExchangeRates) -> Vec<Self> {
        v.into_iter()
            .map(|(cur, rate)| Self {
                code: cur.code(),
                value: rate.0,
            }).collect()
    }
}
