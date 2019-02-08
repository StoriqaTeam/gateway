use juniper::ID as GraphqlID;
use uuid::Uuid;

use super::Attribute;
use stq_static_resources::{Translation, TranslationInput};
use stq_types::CategoryId;

#[derive(Deserialize, Debug, Clone)]
pub struct Category {
    pub id: CategoryId,
    pub slug: String,
    pub name: Vec<Translation>,
    pub meta_field: Option<String>,
    pub children: Vec<Category>,
    pub parent_id: Option<CategoryId>,
    pub level: i32,
    pub attributes: Vec<Attribute>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CategoryWithProducts(pub Category);

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
    #[graphql(description = "Category level (DEPRECATED: this field is now ignored, new level is calculated automatically).")]
    pub level: Option<i32>,
    #[graphql(description = "Category slug.")]
    pub slug: Option<String>,
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
            slug: None,
        } == self.clone()
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create Category input object")]
pub struct CreateCategoryInput {
    #[graphql(name = "clientMutationId", description = "Client mutation id.")]
    pub uuid: String,
    #[graphql(description = "Name of a Category.")]
    pub name: Vec<TranslationInput>,
    #[graphql(description = "Meta field of a category.")]
    pub meta_field: Option<String>,
    #[graphql(description = "Parent category id.")]
    pub parent_id: i32,
    #[graphql(description = "Category slug.")]
    pub slug: Option<String>,
}

impl CreateCategoryInput {
    pub fn fill_uuid(mut self) -> Self {
        self.uuid = Some(self.uuid)
            .filter(|id| !id.is_empty())
            .unwrap_or_else(|| Uuid::new_v4().hyphenated().to_string());
        self
    }
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

/// Payload for deleting category
#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Delete category input object")]
pub struct DeleteCategoryInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of Category.")]
    pub cat_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchCategory(pub Category);

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Change base_product in coupon")]
pub struct CategoryReplaceInput {
    #[graphql(description = "Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a old category.")]
    pub current_category: i32,
    #[graphql(description = "Id of a new category.")]
    pub new_category: i32,
    #[graphql(description = "Ids of a base products.
        When `base_product_ids` equal `null` category will replacing all entries in base products.")]
    pub base_product_ids: Option<Vec<i32>>,
}
