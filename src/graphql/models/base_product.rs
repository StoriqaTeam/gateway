use juniper::ID as GraphqlID;
use stq_static_resources::{Translation, TranslationInput};

use super::*;

#[derive(Deserialize, Debug, Clone)]
pub struct BaseProduct {
    pub id: i32,
    pub is_active: bool,
    pub store_id: i32,
    pub name: Vec<Translation>,
    pub short_description: Vec<Translation>,
    pub long_description: Option<Vec<Translation>>,
    pub seo_title: Option<Vec<Translation>>,
    pub seo_description: Option<Vec<Translation>>,
    pub currency_id: i32,
    pub category_id: i32,
    pub views: i32,
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
    #[graphql(description = "currency_id")]
    pub currency_id: Option<i32>,
    #[graphql(description = "Category id.")]
    pub category_id: Option<i32>,
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
            currency_id: None,
            category_id: None,
        } == self.clone()
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create base_product with attributes input object")]
pub struct CreateBaseProductInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Name of new base_product.")]
    pub name: Vec<TranslationInput>,
    #[graphql(description = "Store id base_product belonging to.")]
    pub store_id: i32,
    #[graphql(description = "Short description")]
    pub short_description: Vec<TranslationInput>,
    #[graphql(description = "Long description")]
    pub long_description: Option<Vec<TranslationInput>>,
    #[graphql(description = "SEO title")]
    pub seo_title: Option<Vec<TranslationInput>>,
    #[graphql(description = "SEO description")]
    pub seo_description: Option<Vec<TranslationInput>>,
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

#[derive(Deserialize, Debug, Clone)]
pub struct BaseProductWithVariants {
    pub base_product: BaseProduct,
    pub variants: Vec<VariantsWithAttributes>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VariantsWithAttributes {
    pub product: Product,
    pub attrs: Vec<AttrValue>,
}
