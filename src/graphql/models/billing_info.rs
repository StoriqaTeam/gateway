use stq_static_resources::Currency;
use stq_types::{Alpha3, InternationalBillingId, ProxyCompanyBillingInfoId, RussiaBillingId, StoreId, SwiftId};

#[derive(Deserialize, Debug, Clone)]
pub struct InternationalBillingInfo {
    pub id: InternationalBillingId,
    pub store_id: StoreId,
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

#[derive(Deserialize, Debug, Clone)]
pub struct RussiaBillingInfo {
    pub id: RussiaBillingId,
    pub store_id: StoreId,
    pub bank_name: String,
    pub branch_name: Option<String>,
    pub swift_bic: SwiftId,
    pub tax_id: String,
    pub correspondent_account: String,
    pub current_account: String,
    pub personal_account: Option<String>,
    pub beneficiary_full_name: String,
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
    #[graphql(description = "account.")]
    pub account: String,
    #[graphql(description = "currency.")]
    pub currency: Currency,
    #[graphql(description = "name.")]
    pub name: String,
    #[graphql(description = "bank.")]
    pub bank: String,
    #[graphql(description = "swift.")]
    pub swift: String,
    #[graphql(description = "bank address.")]
    pub bank_address: String,
    #[graphql(description = "country.")]
    pub country: String,
    #[graphql(description = "city.")]
    pub city: String,
    #[graphql(description = "recipient address.")]
    pub recipient_address: String,
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
    #[graphql(description = "account.")]
    pub account: Option<String>,
    #[graphql(description = "currency.")]
    pub currency: Currency,
    #[graphql(description = "name.")]
    pub name: Option<String>,
    #[graphql(description = "bank.")]
    pub bank: Option<String>,
    #[graphql(description = "swift.")]
    pub swift: Option<String>,
    #[graphql(description = "bank address.")]
    pub bank_address: Option<String>,
    #[graphql(description = "country.")]
    pub country: Option<String>,
    #[graphql(description = "city.")]
    pub city: Option<String>,
    #[graphql(description = "recipient address.")]
    pub recipient_address: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create russia billing info input object")]
pub struct NewRussiaBillingInfoInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "store id.")]
    pub store_id: i32,
    #[graphql(description = "bank name.")]
    pub bank_name: String,
    #[graphql(description = "branch name.")]
    pub branch_name: Option<String>,
    #[graphql(description = "swift bic.")]
    pub swift_bic: String,
    #[graphql(description = "tax id.")]
    pub tax_id: String,
    #[graphql(description = "correspondent account.")]
    pub correspondent_account: String,
    #[graphql(description = "current account.")]
    pub current_account: String,
    #[graphql(description = "personal account.")]
    pub personal_account: Option<String>,
    #[graphql(description = "beneficiary full name.")]
    pub beneficiary_full_name: String,
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
    #[graphql(description = "bank name.")]
    pub bank_name: Option<String>,
    #[graphql(description = "branch name.")]
    pub branch_name: Option<String>,
    #[graphql(description = "swift bic.")]
    pub swift_bic: Option<String>,
    #[graphql(description = "tax id.")]
    pub tax_id: Option<String>,
    #[graphql(description = "correspondent account.")]
    pub correspondent_account: Option<String>,
    #[graphql(description = "current account.")]
    pub current_account: Option<String>,
    #[graphql(description = "personal account.")]
    pub personal_account: Option<String>,
    #[graphql(description = "beneficiary full name.")]
    pub beneficiary_full_name: Option<String>,
}
