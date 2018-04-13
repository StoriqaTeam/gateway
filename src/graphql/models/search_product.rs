use super::*;

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
pub struct ProductsSearchOptionsInput {
    #[graphql(description = "Attribute filters.")]
    pub attr_filters: Option<Vec<AttributeFilterInput>>,
    #[graphql(description = "Price filter.")]
    pub price_filter: Option<RangeFilterInput>,
    #[graphql(description = "Categories ids.")]
    pub category_id: Option<i32>,
}

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
#[graphql(description = "Search product input object")]
pub struct SearchProductInput {
    #[graphql(description = "Name part of the product.")]
    pub name: String,
    #[graphql(description = "Searching options")]
    pub options: Option<ProductsSearchOptionsInput>,
}

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
#[graphql(description = "Search product input object")]
pub struct MostViewedProductsInput {
    #[graphql(description = "Searching options")]
    pub options: Option<ProductsSearchOptionsInput>,
}

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
#[graphql(description = "Search product input object")]
pub struct MostDiscountProductsInput {
    #[graphql(description = "Searching options")]
    pub options: Option<ProductsSearchOptionsInput>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProductsSearchFilters {
    pub search_term: SearchProductInput,
}

impl ProductsSearchFilters {
    pub fn new(search_term: SearchProductInput) -> Self {
        Self {
            search_term
        }
    }
}

