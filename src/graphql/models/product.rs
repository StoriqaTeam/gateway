use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

#[derive(Deserialize, Debug, Clone)]
pub struct Product {
    pub id: i32,
    pub store_id: i32,
    pub name: String,
    pub is_active: bool,
    pub short_description: String,
    pub long_description: Option<String>,
    pub price: f64,
    pub currency_id: i32,
    pub discount: Option<f64>,
    pub category: Option<i32>,
    pub photo_main: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Update product input object")]
pub struct UpdateProductInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a product.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "New name of a product.")]
    pub name: Option<String>,
    #[graphql(description = "currency_id")]
    pub currency_id: Option<i32>,
    #[graphql(description = "short_description")]
    pub short_description: Option<String>,
    #[graphql(description = "long_description")]
    pub long_description: Option<String>,
    #[graphql(description = "price")]
    pub price: Option<f64>,
    #[graphql(description = "discount")]
    pub discount: Option<f64>,
    #[graphql(description = "category")]
    pub category: Option<i32>,
    #[graphql(description = "photo_main")]
    pub photo_main: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create product with attributes input object")]
pub struct CreateProductWithAttributesInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "New Product")]
    pub product: NewProduct,
    #[graphql(description = "Attributes")]
    pub attributes: Vec<AttrValue>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
pub struct NewProduct {
    #[graphql(description = "Name of new product.")]
    pub name: String,
    #[graphql(description = "Store id product belonging to.")]
    pub store_id: i32,
    #[graphql(description = "Sale currency id.")]
    pub currency_id: i32,
    #[graphql(description = "Short description")]
    pub short_description: String,
    #[graphql(description = "Long description")]
    pub long_description: Option<String>,
    #[graphql(description = "Price of the product.")]
    pub price: f64,
    #[graphql(description = "Discount.")]
    pub discount: Option<f64>,
    #[graphql(description = "Main photo of the product.")]
    pub photo_main: Option<String>,
    #[graphql(description = "Vendor code.")]
    pub vendor_code: Option<String>,
    #[graphql(description = "Cashback.")]
    pub cashback: Option<f64>,
    #[graphql(description = "Language of the descriptions.")]
    pub language_id: i32,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
pub struct AttrValue {
    #[graphql(description = "Attribute name")]
    pub name: String,
    #[graphql(description = "Attribute value")]
    pub value: String,
    #[graphql(description = "Attribute type")]
    pub value_type: AttributeType,
}

#[derive(GraphQLEnum, Serialize, Clone, Debug)]
#[serde(tag = "attribute_type")]
pub enum AttributeType {
    Str,
    Float,
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "Deactivate product input object")]
pub struct DeactivateProductInput {
    #[graphql(description = "Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a product.")]
    pub id: GraphqlID,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone)]
#[graphql(description = "Search product input object")]
pub struct SearchProductInput {
    #[graphql(description = "Name part of the product.")]
    pub name: Option<String>,
    #[graphql(description = "Attribute filters.")]
    pub attr_filters: Option<Vec<AttributeFilterInput>>,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug)]
pub struct AttributeFilterInput {
    #[graphql(description = "Attribute name")]
    pub name: String,
    #[graphql(description = "Attribute type")]
    pub filter_type: FilterTypeInput,
    #[graphql(description = "Attribute value")]
    pub value: String,
}

#[derive(GraphQLEnum, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "Filter type. Equal can be used for strings, enums, bool, ints: value will be interpreted as string. Other filters will be applied to float values.")]
pub enum FilterTypeInput {
    Equal,
    Lte,
    Le,
    Ge,
    Gte,
}

#[derive(Serialize, Clone, Debug)]
pub struct SearchProduct {
    pub name: Option<String>,
    pub attr_filters: Option<Vec<AttributeFilter>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct AttributeFilter {
    pub name: String,
    pub filter: Filter,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Filter {
    Equal(String),
    Lte(f32),
    Le(f32),
    Ge(f32),
    Gte(f32),
}

impl AttributeFilter {
    pub fn from_input(attr: AttributeFilterInput) -> FieldResult<Self> {
        let filter = match attr.filter_type {
            FilterTypeInput::Equal => Filter::Equal(attr.value),
            v => {
                let val = attr.value.parse().map_err(|_| {
                    FieldError::new(
                        "Validation error",
                        graphql_value!({ "code": 300, "details": {
                            format!("Can not parse filter value as float.")
                            }}),
                    )
                })?;

                match v {
                    FilterTypeInput::Lte => Filter::Lte(val),
                    FilterTypeInput::Le => Filter::Le(val),
                    FilterTypeInput::Ge => Filter::Ge(val),
                    FilterTypeInput::Gte => Filter::Gte(val),
                    _ => unreachable!(),
                }
            }
        };
        Ok(Self {
            name: attr.name,
            filter: filter,
        })
    }
}

impl SearchProduct {
    pub fn from_input(s: SearchProductInput) -> FieldResult<Self> {
        let filters = match s.attr_filters {
            None => None,
            Some(filters) => {
                let f = filters
                    .into_iter()
                    .map(|filter| AttributeFilter::from_input(filter))
                    .collect::<FieldResult<Vec<AttributeFilter>>>()?;
                Some(f)
            }
        };

        Ok(Self {
            name: s.name,
            attr_filters: filters,
        })
    }
}
