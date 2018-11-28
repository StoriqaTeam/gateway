use std::time::SystemTime;

use juniper::ID as GraphqlID;
use uuid::Uuid;

use stq_static_resources::{Currency, ModerationStatus, Translation, TranslationInput};
use stq_types::{BaseProductId, CategoryId, StoreId};

use super::*;

#[derive(Deserialize, Debug, Clone)]
pub struct BaseProduct {
    pub id: BaseProductId,
    pub is_active: bool,
    pub store_id: StoreId,
    pub name: Vec<Translation>,
    pub short_description: Vec<Translation>,
    pub long_description: Option<Vec<Translation>>,
    pub seo_title: Option<Vec<Translation>>,
    pub seo_description: Option<Vec<Translation>>,
    pub currency: Currency,
    pub category_id: CategoryId,
    pub views: i32,
    pub rating: f64,
    pub slug: String,
    pub status: ModerationStatus,
    pub variants: Option<Vec<Product>>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Update base_product with attributes input object")]
pub struct UpdateBaseProductInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a base_product.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "New name of a base product.")]
    pub name: Option<Vec<TranslationInput>>,
    #[graphql(description = "short description")]
    pub short_description: Option<Vec<TranslationInput>>,
    #[graphql(description = "long description")]
    pub long_description: Option<Vec<TranslationInput>>,
    #[graphql(description = "seo title")]
    pub seo_title: Option<Vec<TranslationInput>>,
    #[graphql(description = "seo description")]
    pub seo_description: Option<Vec<TranslationInput>>,
    #[graphql(description = "Currency")]
    pub currency: Option<Currency>,
    #[graphql(description = "Category id.")]
    pub category_id: Option<i32>,
    #[graphql(description = "Rating.")]
    pub rating: Option<f64>,
    #[graphql(description = "Slug.")]
    pub slug: Option<String>,
}

impl UpdateBaseProductInput {
    pub fn is_none(&self) -> bool {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            id: self.id.clone(),
            name: None,
            short_description: None,
            long_description: None,
            seo_title: None,
            seo_description: None,
            currency: None,
            category_id: None,
            rating: None,
            slug: None,
        } == self.clone()
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create base_product with attributes input object")]
pub struct CreateBaseProductInput {
    #[graphql(name = "clientMutationId", description = "Client mutation id.")]
    pub uuid: String,
    #[graphql(description = "Name of new base_product.")]
    pub name: Vec<TranslationInput>,
    #[graphql(description = "Int Store id base_product belonging to.")]
    pub store_id: i32,
    #[graphql(description = "Short description")]
    pub short_description: Vec<TranslationInput>,
    #[graphql(description = "Long description")]
    pub long_description: Option<Vec<TranslationInput>>,
    #[graphql(description = "SEO title")]
    pub seo_title: Option<Vec<TranslationInput>>,
    #[graphql(description = "SEO description")]
    pub seo_description: Option<Vec<TranslationInput>>,
    #[graphql(description = "Int Sale currency id.")]
    pub currency: Currency,
    #[graphql(description = "Int Category id.")]
    pub category_id: i32,
    #[graphql(description = "Slug.")]
    pub slug: Option<String>,
}

impl CreateBaseProductInput {
    pub fn fill_uuid(mut self) -> Self {
        self.uuid = Some(self.uuid)
            .filter(|id| !id.is_empty())
            .unwrap_or_else(|| Uuid::new_v4().hyphenated().to_string());
        self
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create base product with variant input object")]
pub struct NewBaseProductWithVariantsInput {
    #[graphql(name = "clientMutationId", description = "Client mutation id.")]
    pub uuid: String,
    #[graphql(description = "Name of new base_product.")]
    pub name: Vec<TranslationInput>,
    #[graphql(description = "Int Store id base_product belonging to.")]
    pub store_id: i32,
    #[graphql(description = "Short description")]
    pub short_description: Vec<TranslationInput>,
    #[graphql(description = "Long description")]
    pub long_description: Option<Vec<TranslationInput>>,
    #[graphql(description = "SEO title")]
    pub seo_title: Option<Vec<TranslationInput>>,
    #[graphql(description = "SEO description")]
    pub seo_description: Option<Vec<TranslationInput>>,
    #[graphql(description = "Int Sale currency id.")]
    pub currency: Currency,
    #[graphql(description = "Int Category id.")]
    pub category_id: i32,
    #[graphql(description = "Slug.")]
    pub slug: Option<String>,
    #[graphql(description = "New products with attributes.")]
    pub variants: Vec<CreateProductWithAttributesInput>,
    #[graphql(description = "Selected raw id attributes.")]
    pub selected_attributes: Vec<i32>,
}

impl NewBaseProductWithVariantsInput {
    pub fn fill_uuid(mut self) -> Self {
        self.uuid = Some(self.uuid)
            .filter(|id| !id.is_empty())
            .unwrap_or_else(|| Uuid::new_v4().hyphenated().to_string());
        self
    }
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "Deactivate base_product input object")]
pub struct DeactivateBaseProductInput {
    #[graphql(description = "Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a base_product.")]
    pub id: GraphqlID,
}

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
#[graphql(description = "Search base product by moderator input object")]
pub struct SearchModeratorBaseProductInput {
    #[graphql(description = "Name part of the base product.")]
    pub name: Option<String>,
    #[graphql(description = "Store id of the base product.")]
    pub store_id: Option<i32>,
    #[graphql(description = "Moderation state of the base product.")]
    pub state: Option<ModerationStatus>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BaseProductSearchResults {
    pub total_count: u32,
    pub base_products: Vec<BaseProduct>,
}

#[derive(GraphQLInputObject, Clone, Debug)]
#[graphql(description = "Management base product moderation status input object")]
pub struct BaseProductModerateInput {
    #[graphql(description = "Id of a base product.")]
    pub id: GraphqlID,
    #[graphql(description = "Moderation state of the base product.")]
    pub status: ModerationStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BaseProductModerate {
    pub base_product_id: BaseProductId,
    pub status: ModerationStatus,
}
