use super::*;
use geo::Point;
use juniper::ID as GraphqlID;

use stq_api::warehouses::{Warehouse, WarehouseInput, WarehouseUpdateData};
use stq_types::{Alpha3, StoreId, WarehouseId, WarehouseSlug};

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
pub struct GraphQLWarehouse(pub Warehouse);

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
    #[graphql(description = "Slug of a warehouse.")]
    pub slug: Option<String>,
}

impl UpdateWarehouseInput {
    pub fn is_none(&self) -> bool {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            id: self.id.clone(),
            name: None,
            location: None,
            slug: None,
            address_full: AddressInput {
                country: None,
                country_code: None,
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

impl From<UpdateWarehouseInput> for WarehouseUpdateData {
    fn from(value: UpdateWarehouseInput) -> Self {
        Self {
            slug: value.slug.map(WarehouseSlug).map(From::from),
            name: value.name.map(Some).map(From::from),
            location: value.location.map(|p| Point::new(p.x, p.y)).map(Some).map(From::from),
            administrative_area_level_1: value.address_full.administrative_area_level_1.map(Some).map(From::from),
            administrative_area_level_2: value.address_full.administrative_area_level_2.map(Some).map(From::from),
            country: value.address_full.country.map(Some).map(From::from),
            locality: value.address_full.locality.map(Some).map(From::from),
            political: value.address_full.political.map(Some).map(From::from),
            postal_code: value.address_full.postal_code.map(Some).map(From::from),
            route: value.address_full.route.map(Some).map(From::from),
            street_number: value.address_full.street_number.map(Some).map(From::from),
            address: value.address_full.value.map(Some).map(From::from),
            place_id: value.address_full.place_id.map(Some).map(From::from),
            country_code: value.address_full.country_code.map(Alpha3).map(Some).map(From::from),
        }
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
    #[graphql(description = "Slug of a warehouse.")]
    pub slug: Option<String>,
    #[graphql(description = "Store id of a warehouse.")]
    pub store_id: i32,
    #[graphql(description = "Location of a warehouse.")]
    pub location: Option<GeoPointInput>,
    #[graphql(description = "Address")]
    #[serde(flatten)]
    pub address_full: AddressInput,
}

impl From<CreateWarehouseInput> for WarehouseInput {
    fn from(value: CreateWarehouseInput) -> Self {
        Self {
            id: WarehouseId::new(),
            name: value.name.map(From::from),
            store_id: StoreId(value.store_id),
            location: value.location.map(|p| Point::new(p.x, p.y)),
            administrative_area_level_1: value.address_full.administrative_area_level_1.map(From::from),
            administrative_area_level_2: value.address_full.administrative_area_level_2.map(From::from),
            country: value.address_full.country.map(From::from),
            country_code: value.address_full.country_code.map(From::from),
            locality: value.address_full.locality.map(From::from),
            political: value.address_full.political.map(From::from),
            postal_code: value.address_full.postal_code.map(From::from),
            route: value.address_full.route.map(From::from),
            street_number: value.address_full.street_number.map(From::from),
            address: value.address_full.value.map(From::from),
            place_id: value.address_full.place_id.map(From::from),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PageInfoWarehouseProductSearch {
    pub total_pages: i32,
    pub current_page: i32,
    pub page_items_count: i32,
    pub search_term_options: Option<ProductsSearchFilters>,
}
