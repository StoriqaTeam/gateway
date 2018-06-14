use super::*;
use juniper::ID as GraphqlID;

#[derive(GraphQLEnum, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[graphql(description = "Warehouse kind")]
pub enum WarehouseKind {
    #[serde(rename = "distribution_center")]
    #[graphql(description = "DistributionCenter.")]
    DistributionCenter,
    #[serde(rename = "store")]
    #[graphql(description = "Store.")]
    Store,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description = "Geo point")]
pub struct GeoPoint {
    #[graphql(description = "x.")]
    pub x: f64,
    #[graphql(description = "y.")]
    pub y: f64,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[graphql(description = "Geo point")]
pub struct GeoPointInput {
    #[graphql(description = "x.")]
    pub x: f64,
    #[graphql(description = "y.")]
    pub y: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Warehouse {
    pub id: i32,
    pub name: Option<String>,
    pub store_id: i32,
    pub location: Option<GeoPoint>,
    pub administrative_area_level_1: Option<String>,
    pub administrative_area_level_2: Option<String>,
    pub address: Option<String>,
    pub country: Option<String>,
    pub locality: Option<String>,
    pub political: Option<String>,
    pub postal_code: Option<String>,
    pub route: Option<String>,
    pub street_number: Option<String>,
    pub place_id: Option<String>,
    pub admins: Vec<i32>,
    pub managers: Vec<i32>,
    pub kind: WarehouseKind,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Update warehouse input object")]
pub struct UpdateWarehouseInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a warehouse.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "Name of a warehouse.")]
    pub name: Option<String>,
    #[graphql(description = "Location of a warehouse.")]
    pub location: Option<GeoPointInput>,
    #[graphql(description = "Address")]
    #[serde(flatten)]
    pub address_full: AddressInput,
    #[graphql(description = "Warehouse kind")]
    pub kind: Option<WarehouseKind>,
}

impl UpdateWarehouseInput {
    pub fn is_none(&self) -> bool {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            id: self.id.clone(),
            name: None,
            location: None,
            kind: None,
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

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create warehouse input object")]
pub struct CreateWarehouseInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Name of a warehouse.")]
    pub name: Option<String>,
    #[graphql(description = "Store id of a warehouse.")]
    pub store_id: i32,
    #[graphql(description = "Location of a warehouse.")]
    pub location: Option<GeoPointInput>,
    #[graphql(description = "Address")]
    #[serde(flatten)]
    pub address_full: AddressInput,
    #[graphql(description = "Warehouse kind")]
    pub kind: WarehouseKind,
}
