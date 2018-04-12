use super::*;

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
pub struct SearchOptionsInput {
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
    pub options: Option<SearchOptionsInput>,
}

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
#[graphql(description = "Search product input object")]
pub struct MostViewedProductsInput {
    #[graphql(description = "Searching options")]
    pub options: Option<SearchOptionsInput>,
}

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
#[graphql(description = "Search product input object")]
pub struct MostDiscountProductsInput {
    #[graphql(description = "Searching options")]
    pub options: Option<SearchOptionsInput>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SearchFilters {
    pub price_range: Option<RangeFilter>,
    pub attr_filters: Option<Vec<AttributeFilter>>,
    pub categories: Category,
}
