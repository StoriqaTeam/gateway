use super::*;

#[derive(GraphQLInputObject, Serialize, Clone, Debug, Default)]
pub struct ProductsSearchOptionsInput {
    #[graphql(description = "Attribute filters.")]
    pub attr_filters: Option<Vec<AttributeFilterInput>>,
    #[graphql(description = "Price filter.")]
    pub price_filter: Option<RangeFilterInput>,
    #[graphql(description = "Currency id.")]
    pub currency_id: Option<i32>,
    #[graphql(description = "Category id.")]
    pub category_id: Option<i32>,
    #[graphql(description = "Store id.")]
    pub store_id: Option<i32>,
    #[graphql(description = "Sorting.")]
    pub sort_by: Option<ProductsSorting>,
}

#[derive(GraphQLInputObject, Serialize, Clone, Debug, Default)]
#[graphql(description = "Search product input object")]
pub struct SearchProductInput {
    #[graphql(description = "Name part of the product.")]
    pub name: String,
    #[graphql(description = "Searching options")]
    pub options: Option<ProductsSearchOptionsInput>,
}

#[derive(GraphQLInputObject, Serialize, Clone, Debug, Default)]
#[graphql(description = "Auto complete product name input object")]
pub struct AutoCompleteProductNameInput {
    #[graphql(description = "Name part of the product.")]
    pub name: String,
    #[graphql(description = "Store id.")]
    pub store_id: Option<i32>,
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
        Self { search_term }
    }
}

#[derive(GraphQLEnum, Serialize, Deserialize, Clone, Debug)]
pub enum ProductsSorting {
    Views,
    PriceAsc,
    PriceDesc,
    Discount,
}
