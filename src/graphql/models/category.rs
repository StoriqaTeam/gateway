use juniper::ID as GraphqlID;
use stq_static_resources::{Translation, TranslationInput};

#[derive(Deserialize, Debug, Clone)]
pub struct Category {
    pub id: i32,
    pub name: Vec<Translation>,
    pub meta_field: Option<String>,
    pub children: Vec<Category>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Update Category input object")]
pub struct UpdateCategoryInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a Category.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "Name of a category.")]
    pub name: Option<Vec<TranslationInput>>,
    #[graphql(description = "Meta field of a category.")]
    pub meta_field: Option<String>,
    #[graphql(description = "Parent category id.")]
    pub parent_id: Option<i32>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create Category input object")]
pub struct CreateCategoryInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Name of a Category.")]
    pub name: Vec<TranslationInput>,
    #[graphql(description = "Meta field of a category.")]
    pub meta_field: Option<String>,
    #[graphql(description = "Parent category id.")]
    pub parent_id: Option<i32>,
}
