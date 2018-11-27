use graphql::models::*;
use stq_types::PackageId;

use juniper::ID as GraphqlID;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Packages {
    pub id: PackageId,
    pub name: String,
    pub max_size: i32,
    pub min_size: i32,
    pub max_weight: i32,
    pub min_weight: i32,
    pub deliveries_to: Vec<Country>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "New Packages input object")]
pub struct NewPackagesInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "name")]
    pub name: String,
    #[graphql(description = "max volume (cm^3)")]
    pub max_size: i32,
    #[graphql(description = "min volume (cm^3)")]
    pub min_size: i32,
    #[graphql(description = "max weight (g)")]
    pub max_weight: i32,
    #[graphql(description = "min weight (g)")]
    pub min_weight: i32,
    #[graphql(description = "deliveries_to")]
    pub deliveries_to: Vec<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Update Packages input object")]
pub struct UpdatePackagesInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a Packages.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "name")]
    pub name: Option<String>,
    #[graphql(description = "max volume (cm^3)")]
    pub max_size: Option<i32>,
    #[graphql(description = "min volume (cm^3)")]
    pub min_size: Option<i32>,
    #[graphql(description = "max weight (g)")]
    pub max_weight: Option<i32>,
    #[graphql(description = "min weight (g)")]
    pub min_weight: Option<i32>,
    #[graphql(description = "deliveries_to")]
    pub deliveries_to: Option<Vec<String>>,
}

impl UpdatePackagesInput {
    pub fn is_none(&self) -> bool {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            id: self.id.clone(),
            name: None,
            max_size: None,
            min_size: None,
            max_weight: None,
            min_weight: None,
            deliveries_to: None,
        } == self.clone()
    }
}
