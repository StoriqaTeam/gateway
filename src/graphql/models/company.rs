use juniper::ID as GraphqlID;

use stq_static_resources::Currency;
use stq_types::CompanyId;

use graphql::models::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Company {
    pub id: CompanyId,
    pub name: String,
    pub label: String,
    pub description: Option<String>,
    pub deliveries_from: Vec<Country>,
    pub currency: String,
    pub logo: String,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "New Company input object")]
pub struct NewCompanyInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "name")]
    pub name: String,
    #[graphql(description = "label")]
    pub label: String,
    #[graphql(description = "description")]
    pub description: Option<String>,
    #[graphql(description = "deliveries_from")]
    pub deliveries_from: Vec<String>,
    #[graphql(description = "currency")]
    pub currency: String,
    #[graphql(description = "logo")]
    pub logo: String,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Update Company input object")]
pub struct UpdateCompanyInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a Company.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "name")]
    pub name: Option<String>,
    #[graphql(description = "label")]
    pub label: Option<String>,
    #[graphql(description = "description")]
    pub description: Option<String>,
    #[graphql(description = "deliveries_from")]
    pub deliveries_from: Option<Vec<String>>,
    #[graphql(description = "currency")]
    pub currency: String,
    #[graphql(description = "logo")]
    pub logo: Option<String>,
}

impl UpdateCompanyInput {
    pub fn is_none(&self) -> bool {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            id: self.id.clone(),
            name: None,
            label: None,
            description: None,
            deliveries_from: None,
            currency: Currency::STQ.code().to_owned(),
            logo: None,
        } == self.clone()
    }
}
