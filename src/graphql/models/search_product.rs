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

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
#[graphql(description = "Search product input object")]
pub struct SearchProductsByNameInput {
    #[graphql(description = "Name part of the product.")]
    pub name: String,
    #[graphql(description = "Searching options")]
    pub options: Option<SearchOptionsInput>,
    #[serde(skip_serializing)]
    #[graphql(description = "Get search filters in result")]
    pub get_search_filters: bool,
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
pub struct SearchOptions {
    pub attr_filters: Vec<AttributeFilter>,
    pub price_filter: Option<RangeFilter>,
    pub categories_ids: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct SearchProductResult {
    pub base_product_with_variants: Connection<BaseProductWithVariants>,
    pub search_filters: Option<SearchOptions>,
}
