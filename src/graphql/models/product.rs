use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use stq_static_resources::{Translation, TranslationInput};

#[derive(Deserialize, Debug, Clone)]
pub struct Product {
    pub id: i32,
    pub store_id: i32,
    pub name: Vec<Translation>,
    pub is_active: bool,
    pub short_description: Vec<Translation>,
    pub long_description: Option<Vec<Translation>>,
    pub price: f64,
    pub currency_id: i32,
    pub discount: Option<f64>,
    pub photo_main: Option<String>,
    pub vendor_code: Option<String>,
    pub cashback: Option<f64>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Update product with attributes input object")]
pub struct UpdateProductWithAttributesInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a product.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "Update Product")]
    pub product: UpdateProduct,
    #[graphql(description = "Attributes")]
    pub attributes: Vec<AttrValue>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Update product input object")]
pub struct UpdateProduct {
    #[graphql(description = "New name of a product.")]
    pub name: Option<Vec<TranslationInput>>,
    #[graphql(description = "currency_id")]
    pub currency_id: Option<i32>,
    #[graphql(description = "short_description")]
    pub short_description: Option<Vec<TranslationInput>>,
    #[graphql(description = "long_description")]
    pub long_description: Option<Vec<TranslationInput>>,
    #[graphql(description = "price")]
    pub price: Option<f64>,
    #[graphql(description = "discount")]
    pub discount: Option<f64>,
    #[graphql(description = "photo_main")]
    pub photo_main: Option<String>,
    #[graphql(description = "vendor code")]
    pub vendor_code: Option<String>,
    #[graphql(description = "cashback")]
    pub cashback: Option<f64>,
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
#[graphql(description = "New Product")]
pub struct NewProduct {
    #[graphql(description = "Name of new product.")]
    pub name: Vec<TranslationInput>,
    #[graphql(description = "Store id product belonging to.")]
    pub store_id: i32,
    #[graphql(description = "Sale currency id.")]
    pub currency_id: i32,
    #[graphql(description = "Short description")]
    pub short_description: Vec<TranslationInput>,
    #[graphql(description = "Long description")]
    pub long_description: Option<Vec<TranslationInput>>,
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
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(name = "AttributeValue", description = "Product attributes with values")]
pub struct AttrValue {
    #[graphql(description = "Attribute name")]
    pub name: String,
    #[graphql(description = "Attribute value")]
    pub value: String,
    #[graphql(description = "Attribute type")]
    pub value_type: AttributeType,
    #[graphql(description = "Meta field")]
    pub meta_field: Option<String>,
}

#[derive(GraphQLEnum, Serialize, Clone, Debug)]
#[graphql(name = "AttributeType", description = "Attribute Type")]
#[serde(tag = "attribute_type")]
pub enum AttributeType {
    #[graphql(description = "String type. Can represent enums, bool, int and strings.")]
    Str,
    #[graphql(description = "Float type.")]
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
    pub name: String,
    #[graphql(description = "Attribute filters.")]
    pub attr_filters: Vec<AttributeFilterInput>,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "Attribute Filter")]
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
    #[graphql(description = "Equal")]
    Equal,
    #[graphql(description = "Less than Equal")]
    Lte,
    #[graphql(description = "Less or Equal")]
    Le,
    #[graphql(description = "Greater or Equal")]
    Ge,
    #[graphql(description = "Greater than Equal")]
    Gte,
}

#[derive(Serialize, Clone, Debug)]
pub struct SearchProduct {
    pub name: String,
    pub attr_filters: Vec<AttributeFilter>,
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
        let filters = s.attr_filters
            .into_iter()
            .map(|filter| AttributeFilter::from_input(filter))
            .collect::<FieldResult<Vec<AttributeFilter>>>()?;

        Ok(Self {
            name: s.name,
            attr_filters: filters,
        })
    }
}
