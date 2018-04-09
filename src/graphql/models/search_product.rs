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
pub struct SearchProductInsideCategoryInput {
    #[graphql(description = "Name part of the product.")]
    pub name: String,
    #[graphql(description = "Category id")]
    pub category_id: i32,
    #[graphql(description = "Attribute filters.")]
    pub attr_filters: Vec<AttributeFilterInput>,
    #[graphql(description = "Price filter.")]
    pub price_range: Option<RangeFilterInput>,
}

#[derive(GraphQLInputObject, Serialize, Clone, Debug)]
#[graphql(description = "Search product input object")]
pub struct SearchProductWithoutCategoryInput {
    #[graphql(description = "Name part of the product.")]
    pub name: String,
    #[graphql(description = "Price Range filter")]
    pub price_range: Option<RangeFilterInput>,
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
    pub price_range: Option<RangeFilter>,
    pub categories_ids: Vec<i32>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SearchFiltersWithoutCategory {
    pub price_range: Option<RangeFilter>,
    pub categories: Category,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SearchFiltersInCategory {
    pub attr_filters: Vec<AttributeFilter>,
    pub price_range: Option<RangeFilter>,
}
