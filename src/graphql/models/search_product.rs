use super::*;

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
pub struct SearchOptionsInput {
    #[graphql(description = "Attribute filters.")]
    pub attr_filters: Vec<AttributeFilterInput>,
    #[graphql(description = "Price filter.")]
    pub price_filter: Option<RangeFilterInput>,
    #[graphql(description = "Categories ids.")]
    pub categories_ids: Vec<i32>,
}


#[derive(GraphQLObject, Deserialize, Clone, Debug)]
pub struct SearchOptions {
    #[graphql(description = "Attribute filters.")]
    pub attr_filters: Vec<AttributeFilter>,
    #[graphql(description = "Price filter.")]
    pub price_filter: Option<RangeFilter>,
    #[graphql(description = "Categories ids.")]
    pub categories_ids: Vec<i32>,
}

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
#[graphql(description = "Search product input object")]
pub struct SearchProductsByNameInput {
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
