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
    #[graphql(description = "etherium")]
    pub etherium: CurrencyExchangeValue,
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
    #[graphql(description = "etherium")]
    pub etherium: CurrencyExchangeValueInput,
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
    #[graphql(description = "etherium")]
    pub etherium: f64,
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
    #[graphql(description = "etherium")]
    pub etherium: f64,
    #[graphql(description = "stq")]
    pub stq: f64,
}
