use stq_api::orders::AddressFull as OrdersAddressFull;

use super::*;

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description = "Address")]
pub struct Address {
    #[graphql(description = "Full address")]
    pub value: Option<String>,
    #[graphql(description = "Country")]
    pub country: Option<String>,
    #[graphql(description = "administrative_area_level_1")]
    pub administrative_area_level_1: Option<String>,
    #[graphql(description = "administrative_area_level_2")]
    pub administrative_area_level_2: Option<String>,
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
    #[graphql(description = "place id")]
    pub place_id: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[graphql(description = "Address input object")]
pub struct AddressInput {
    #[graphql(description = "Full address")]
    #[serde(rename = "address")]
    pub value: Option<String>,
    #[graphql(description = "Country")]
    pub country: Option<String>,
    #[graphql(description = "administrative_area_level_1")]
    pub administrative_area_level_1: Option<String>,
    #[graphql(description = "administrative_area_level_2")]
    pub administrative_area_level_2: Option<String>,
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
    #[graphql(description = "place id")]
    pub place_id: Option<String>,
}

impl From<Store> for Address {
    fn from(store: Store) -> Address {
        Self {
            value: store.address,
            country: store.country,
            administrative_area_level_1: store.administrative_area_level_1,
            administrative_area_level_2: store.administrative_area_level_2,
            locality: store.locality,
            political: store.political,
            postal_code: store.postal_code,
            route: store.route,
            street_number: store.street_number,
            place_id: store.place_id,
        }
    }
}

impl From<UserDeliveryAddress> for Address {
    fn from(address: UserDeliveryAddress) -> Address {
        Self {
            value: address.address,
            country: Some(address.country),
            administrative_area_level_1: address.administrative_area_level_1,
            administrative_area_level_2: address.administrative_area_level_2,
            locality: address.locality,
            political: address.political,
            postal_code: Some(address.postal_code),
            route: address.route,
            street_number: address.street_number,
            place_id: address.place_id,
        }
    }
}

impl From<WizardStore> for Address {
    fn from(address: WizardStore) -> Address {
        Self {
            value: address.address,
            country: address.country,
            administrative_area_level_1: address.administrative_area_level_1,
            administrative_area_level_2: address.administrative_area_level_2,
            locality: address.locality,
            political: address.political,
            postal_code: address.postal_code,
            route: address.route,
            street_number: address.street_number,
            place_id: address.place_id,
        }
    }
}

impl From<Warehouse> for Address {
    fn from(address: Warehouse) -> Address {
        Self {
            value: address.address,
            country: address.country,
            administrative_area_level_1: address.administrative_area_level_1,
            administrative_area_level_2: address.administrative_area_level_2,
            locality: address.locality,
            political: address.political,
            postal_code: address.postal_code,
            route: address.route,
            street_number: address.street_number,
            place_id: address.place_id,
        }
    }
}

impl From<OrdersAddressFull> for Address {
    fn from(address: OrdersAddressFull) -> Address {
        Self {
            value: address.address,
            country: address.country,
            administrative_area_level_1: address.administrative_area_level_1,
            administrative_area_level_2: address.administrative_area_level_2,
            locality: address.locality,
            political: address.political,
            postal_code: address.postal_code,
            route: address.route,
            street_number: address.street_number,
            place_id: address.place_id,
        }
    }
}
