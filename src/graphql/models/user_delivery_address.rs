use juniper::ID as GraphqlID;

use stq_types::UserId;

use super::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UserDeliveryAddress {
    pub id: i32,
    pub user_id: UserId,
    pub administrative_area_level_1: Option<String>,
    pub administrative_area_level_2: Option<String>,
    pub country: String,
    pub locality: Option<String>,
    pub political: Option<String>,
    pub postal_code: String,
    pub route: Option<String>,
    pub street_number: Option<String>,
    pub address: Option<String>,
    pub place_id: Option<String>,
    pub is_priority: bool,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[graphql(description = "New user delivery address input object")]
pub struct NewUserDeliveryAddressInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "User id")]
    pub user_id: i32,
    #[graphql(description = "administrative_area_level_1")]
    pub administrative_area_level_1: Option<String>,
    #[graphql(description = "administrative_area_level_2")]
    pub administrative_area_level_2: Option<String>,
    #[graphql(description = "country")]
    pub country: String,
    #[graphql(description = "locality")]
    pub locality: Option<String>,
    #[graphql(description = "political")]
    pub political: Option<String>,
    #[graphql(description = "postal_code")]
    pub postal_code: String,
    #[graphql(description = "route")]
    pub route: Option<String>,
    #[graphql(description = "street_number")]
    pub street_number: Option<String>,
    #[graphql(description = "address")]
    pub address: Option<String>,
    #[graphql(description = "place id")]
    pub place_id: Option<String>,
    #[graphql(description = "is_priority")]
    pub is_priority: bool,
}

#[derive(GraphQLInputObject, Serialize, Clone, Debug, PartialEq, Eq)]
#[graphql(description = "Update user delivery address input object")]
pub struct UpdateUserDeliveryAddressInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of delivery address.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "administrative_area_level_1")]
    pub administrative_area_level_1: Option<String>,
    #[graphql(description = "administrative_area_level_2")]
    pub administrative_area_level_2: Option<String>,
    #[graphql(description = "country")]
    pub country: Option<String>,
    #[graphql(description = "locality")]
    pub locality: Option<String>,
    #[graphql(description = "political")]
    pub political: Option<String>,
    #[graphql(description = "postal_code")]
    pub postal_code: Option<String>,
    #[graphql(description = "route")]
    pub route: Option<String>,
    #[graphql(description = "street_number")]
    pub street_number: Option<String>,
    #[graphql(description = "address")]
    pub address: Option<String>,
    #[graphql(description = "place id")]
    pub place_id: Option<String>,
    #[graphql(description = "is_priority")]
    pub is_priority: Option<bool>,
}

impl UpdateUserDeliveryAddressInput {
    pub fn is_none(&self) -> bool {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            id: self.id.clone(),
            administrative_area_level_1: None,
            administrative_area_level_2: None,
            country: None,
            locality: None,
            political: None,
            postal_code: None,
            route: None,
            street_number: None,
            address: None,
            is_priority: None,
            place_id: None,
        } == self.clone()
    }
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[graphql(description = "New user delivery address input object")]
pub struct NewUserDeliveryAddressFullInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "User id")]
    pub user_id: i32,
    #[graphql(description = "Address")]
    #[serde(flatten)]
    pub address_full: AddressInput,
    #[graphql(description = "is_priority")]
    pub is_priority: bool,
}

#[derive(GraphQLInputObject, Serialize, Clone, Debug, PartialEq)]
#[graphql(description = "Update user delivery address input object")]
pub struct UpdateUserDeliveryAddressFullInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of delivery address.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "Address")]
    #[serde(flatten)]
    pub address_full: AddressInput,
    #[graphql(description = "is_priority")]
    pub is_priority: Option<bool>,
}

impl UpdateUserDeliveryAddressFullInput {
    pub fn is_none(&self) -> bool {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            id: self.id.clone(),
            is_priority: None,
            address_full: AddressInput {
                country: None,
                administrative_area_level_1: None,
                administrative_area_level_2: None,
                locality: None,
                political: None,
                postal_code: None,
                route: None,
                street_number: None,
                value: None,
                place_id: None,
            },
        } == self.clone()
    }
}
