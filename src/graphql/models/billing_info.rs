use stq_static_resources::Currency;
use stq_types::{Alpha3, InternationalBillingId, ProxyCompanyBillingInfoId, RussiaBillingId, StoreId, SwiftId};

#[derive(Deserialize, Debug, Clone)]
pub struct InternationalBillingInfo {
    pub id: InternationalBillingId,
    pub store_id: StoreId,
    pub swift_bic: SwiftId,
    pub bank_name: String,
    pub full_name: String,
    pub iban: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RussiaBillingInfo {
    pub id: RussiaBillingId,
    pub store_id: StoreId,
    pub kpp: String,
    pub bic: String,
    pub inn: String,
    pub full_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProxyCompanyBillingInfo {
    pub id: ProxyCompanyBillingInfoId,
    pub country_alpha3: Alpha3,
    pub account: String,
    pub currency: Currency,
    pub name: String,
    pub bank: String,
    pub swift: SwiftId,
    pub bank_address: String,
    pub country: String,
    pub city: String,
    pub recipient_address: String,
}

#[derive(GraphQLEnum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[graphql(name = "BillingType", description = "Billing type")]
pub enum BillingType {
    #[graphql(description = "International billing type.")]
    International,
    #[graphql(description = "Russian local billing type.")]
    Russia,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create international billing info input object")]
pub struct NewInternationalBillingInfoInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "store id.")]
    pub store_id: i32,
    #[graphql(description = "swift bic.")]
    pub swift_bic: String,
    #[graphql(description = "bank name.")]
    pub bank_name: String,
    #[graphql(description = "full name.")]
    pub full_name: String,
    #[graphql(description = "iban.")]
    pub iban: String,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create international billing info input object")]
pub struct UpdateInternationalBillingInfoInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of an international billing info.")]
    #[serde(skip_serializing)]
    pub id: i32,
    #[graphql(description = "store id.")]
    pub store_id: Option<i32>,
    #[graphql(description = "swift bic.")]
    pub swift_bic: Option<String>,
    #[graphql(description = "bank name.")]
    pub bank_name: Option<String>,
    #[graphql(description = "full name.")]
    pub full_name: Option<String>,
    #[graphql(description = "iban.")]
    pub iban: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create russia billing info input object")]
pub struct NewRussiaBillingInfoInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "store id.")]
    pub store_id: i32,
    #[graphql(description = "bic.")]
    pub kpp: String,
    #[graphql(description = "bic.")]
    pub bic: String,
    #[graphql(description = "inn.")]
    pub inn: String,
    #[graphql(description = "full name.")]
    pub full_name: String,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create russia billing info input object")]
pub struct UpdateRussiaBillingInfoInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of an russia billing info.")]
    #[serde(skip_serializing)]
    pub id: i32,
    #[graphql(description = "store id.")]
    pub store_id: Option<i32>,
    #[graphql(description = "bic.")]
    pub kpp: Option<String>,
    #[graphql(description = "bic.")]
    pub bic: Option<String>,
    #[graphql(description = "inn.")]
    pub inn: Option<String>,
    #[graphql(description = "full name.")]
    pub full_name: Option<String>,
}
