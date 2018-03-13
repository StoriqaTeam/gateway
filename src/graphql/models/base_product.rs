use juniper::ID as GraphqlID;

use stq_static_resources::{Translation, TranslationInput};

#[derive(Deserialize, Debug, Clone)]
pub struct BaseProduct {
    pub id: i32,
    pub is_active: bool,
    pub store_id: i32,
    pub name: Vec<Translation>,
    pub short_description: Vec<Translation>,
    pub long_description: Option<Vec<Translation>>,
    pub currency_id: i32,
    pub category_id: i32,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Update base_product with attributes input object")]
pub struct UpdateBaseProductInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a base_product.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "Update Base Product")]
    pub base_product: UpdateBaseProduct,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Update base_product input object")]
pub struct UpdateBaseProduct {
    #[graphql(description = "New name of a base product.")]
    pub name: Option<Vec<TranslationInput>>,
    #[graphql(description = "short description")]
    pub short_description: Option<Vec<TranslationInput>>,
    #[graphql(description = "long description")]
    pub long_description: Option<Vec<TranslationInput>>,
    #[graphql(description = "currency_id")]
    pub currency_id: Option<i32>,
    #[graphql(description = "Category id.")]
    pub category_id: Option<i32>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create base_product with attributes input object")]
pub struct CreateBaseProductInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "New BaseProduct")]
    pub base_product: NewBaseProduct,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "New BaseProduct")]
pub struct NewBaseProduct {
    #[graphql(description = "Name of new base_product.")]
    pub name: Vec<TranslationInput>,
    #[graphql(description = "Store id base_product belonging to.")]
    pub store_id: i32,
    #[graphql(description = "Short description")]
    pub short_description: Vec<TranslationInput>,
    #[graphql(description = "Long description")]
    pub long_description: Option<Vec<TranslationInput>>,
    #[graphql(description = "Sale currency id.")]
    pub currency_id: i32,
    #[graphql(description = "Category id.")]
    pub category_id: i32,
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "Deactivate base_product input object")]
pub struct DeactivateBaseProductInput {
    #[graphql(description = "Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a base_product.")]
    pub id: GraphqlID,
}
