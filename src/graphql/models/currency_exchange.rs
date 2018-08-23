use stq_static_resources::Currency;

#[derive(GraphQLObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "Currency exchange input object")]
pub struct CurrencyExchange {
    #[graphql(description = "rouble")]
    pub rouble: CurrencyExchangeValue,
    #[graphql(description = "euro")]
    pub euro: CurrencyExchangeValue,
    #[graphql(description = "dollar")]
    pub dollar: CurrencyExchangeValue,
    #[graphql(description = "bitcoin")]
    pub bitcoin: CurrencyExchangeValue,
    #[graphql(description = "ether")]
    pub ether: CurrencyExchangeValue,
    #[graphql(description = "stq")]
    pub stq: CurrencyExchangeValue,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "New currency exchange input object")]
pub struct NewCurrencyExchangeInput {
    #[graphql(description = "rouble")]
    pub rouble: CurrencyExchangeValueInput,
    #[graphql(description = "euro")]
    pub euro: CurrencyExchangeValueInput,
    #[graphql(description = "dollar")]
    pub dollar: CurrencyExchangeValueInput,
    #[graphql(description = "bitcoin")]
    pub bitcoin: CurrencyExchangeValueInput,
    #[graphql(description = "ether")]
    pub ether: CurrencyExchangeValueInput,
    #[graphql(description = "stq")]
    pub stq: CurrencyExchangeValueInput,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "Currency exchange value object")]
pub struct CurrencyExchangeValueInput {
    #[graphql(description = "rouble")]
    pub rouble: f64,
    #[graphql(description = "euro")]
    pub euro: f64,
    #[graphql(description = "dollar")]
    pub dollar: f64,
    #[graphql(description = "bitcoin")]
    pub bitcoin: f64,
    #[graphql(description = "ether")]
    pub ether: f64,
    #[graphql(description = "stq")]
    pub stq: f64,
}

#[derive(GraphQLObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "Currency exchange value object")]
pub struct CurrencyExchangeValue {
    #[graphql(description = "rouble")]
    pub rouble: f64,
    #[graphql(description = "euro")]
    pub euro: f64,
    #[graphql(description = "dollar")]
    pub dollar: f64,
    #[graphql(description = "bitcoin")]
    pub bitcoin: f64,
    #[graphql(description = "ether")]
    pub ether: f64,
    #[graphql(description = "stq")]
    pub stq: f64,
}

#[derive(GraphQLObject, Serialize, Deserialize, Clone, Debug, Default)]
pub struct CurrencyExchange2 {
    pub key: i32,
    pub rates: Vec<CurrencyExchangeValue2>,
}

impl CurrencyExchange2 {
    pub fn from_v1(v: CurrencyExchange) -> Vec<CurrencyExchange2> {
        vec![
            Self {
                key: Currency::RUB as i32,
                rates: CurrencyExchangeValue2::from_v1(v.rouble),
            },
            Self {
                key: Currency::EUR as i32,
                rates: CurrencyExchangeValue2::from_v1(v.euro),
            },
            Self {
                key: Currency::USD as i32,
                rates: CurrencyExchangeValue2::from_v1(v.dollar),
            },
            Self {
                key: Currency::BTC as i32,
                rates: CurrencyExchangeValue2::from_v1(v.bitcoin),
            },
            Self {
                key: Currency::ETH as i32,
                rates: CurrencyExchangeValue2::from_v1(v.ether),
            },
            Self {
                key: Currency::STQ as i32,
                rates: CurrencyExchangeValue2::from_v1(v.stq),
            },
        ]
    }
}

#[derive(GraphQLObject, Serialize, Deserialize, Clone, Debug)]
pub struct CurrencyExchangeValue2 {
    pub key: i32,
    pub value: f64,
}

impl CurrencyExchangeValue2 {
    pub fn from_v1(v: CurrencyExchangeValue) -> Vec<CurrencyExchangeValue2> {
        vec![
            Self {
                key: Currency::RUB as i32,
                value: v.rouble,
            },
            Self {
                key: Currency::EUR as i32,
                value: v.euro,
            },
            Self {
                key: Currency::USD as i32,
                value: v.dollar,
            },
            Self {
                key: Currency::BTC as i32,
                value: v.bitcoin,
            },
            Self {
                key: Currency::ETH as i32,
                value: v.ether,
            },
            Self {
                key: Currency::STQ as i32,
                value: v.stq,
            },
        ]
    }
}
