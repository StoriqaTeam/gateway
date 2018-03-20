use juniper::FieldResult;
use super::*;

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone)]
#[graphql(description = "Search product input object")]
pub struct SearchProductNameInput {
    #[graphql(description = "Name part of the product.")]
    pub name: String,
    #[graphql(description = "Attribute filters.")]
    pub attr_filters: Vec<AttributeFilterInput>,
    #[graphql(description = "Categories ids.")]
    pub categories_ids: Vec<i32>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SearchProductName {
    pub name: String,
    pub attr_filters: Vec<AttributeFilter>,
    pub categories_ids: Vec<i32>,
}

impl SearchProductName {
    pub fn from_input(s: SearchProductNameInput) -> FieldResult<Self> {
        let filters = s.attr_filters
            .into_iter()
            .map(|filter| AttributeFilter::from_input(filter))
            .collect::<FieldResult<Vec<AttributeFilter>>>()?;

        Ok(Self {
            name: s.name,
            attr_filters: filters,
            categories_ids: s.categories_ids
        })
    }
}
