//! EAV model attributes
use stq_static_resources::{Translation, TranslationInput};
use juniper::ID as GraphqlID;

#[derive(Deserialize, Debug, Clone)]
pub struct Attribute {
    pub id: i32,
    pub name: Vec<Translation>,
    pub meta_field: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Update attribute input object")]
pub struct UpdateAttributeInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a attribute.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "New name of an attribute")]
    pub name: Option<Vec<TranslationInput>>,
    #[graphql(description = "New meta_field of an attribute")]
    pub meta_field: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create attribute input object")]
pub struct CreateAttributeInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Name of an attribute.")]
    pub name: Vec<TranslationInput>,
    #[graphql(description = "Meta field of an attribute.")]
    pub meta_field: Option<String>,
}
