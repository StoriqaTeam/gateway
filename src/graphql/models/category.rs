use super::Attribute;
use juniper::ID as GraphqlID;
use stq_static_resources::{Translation, TranslationInput};

#[derive(Deserialize, Debug, Clone)]
pub struct Category {
    pub id: i32,
    pub name: Vec<Translation>,
    pub meta_field: Option<String>,
    pub children: Vec<Category>,
    pub parent_id: Option<i32>,
    pub level: i32,
    pub attributes: Vec<Attribute>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
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
    #[graphql(description = "Category level.")]
    pub level: Option<i32>,
}

impl UpdateCategoryInput {
    pub fn is_none(&self) -> bool {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            id: self.id.clone(),
            name: None,
            meta_field: None,
            parent_id: None,
            level: None,
        } == self.clone()
    }
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
    #[graphql(description = "Category level.")]
    pub level: i32,
}

/// Payload for adding category attributes
#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Add Attribute To Category Input input object")]
pub struct AddAttributeToCategoryInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of Category.")]
    pub cat_id: i32,
    #[graphql(description = "Id of Attribute.")]
    pub attr_id: i32,
}

/// Payload for deleting category attributes
#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Delete Attribute To Category Input input object")]
pub struct DeleteAttributeFromCategory {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of Category.")]
    pub cat_id: i32,
    #[graphql(description = "Id of Attribute.")]
    pub attr_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchCategory(pub Category);
