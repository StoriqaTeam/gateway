//! EAV model attributes
use stq_static_resources::{Translation, TranslationInput};
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

#[derive(Deserialize, Debug, Clone)]
pub struct Attribute {
    pub id: i32,
    pub name: Vec<Translation>,
    pub meta_field: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Update attribute input object")]
pub struct UpdateAttributeInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a attribute.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "New name of an attribute")]
    pub name: Option<Vec<TranslationInput>>,
    #[graphql(description = "New meta_field of an attribute")]
    pub meta_field: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create attribute input object")]
pub struct CreateAttributeInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Name of an attribute.")]
    pub name: Vec<TranslationInput>,
    #[graphql(description = "Meta field of an attribute.")]
    pub meta_field: Option<String>,
}

#[derive(GraphQLInputObject, Deserialize, Serialize, Debug, Clone)]
#[graphql(name = "AttrValueInput", description = "Product attributes with values input object")]
pub struct AttrValueInput {
    #[graphql(description = "Attribute id")]
    pub attr_id: i32,
    #[graphql(description = "Attribute value")]
    pub value: String,
    #[graphql(description = "Attribute type")]
    pub value_type: AttributeType,
    #[graphql(description = "Meta field")]
    pub meta_field: Option<String>,
}

#[derive(GraphQLObject, Deserialize, Serialize, Debug, Clone)]
#[graphql(name = "AttributeValue", description = "Product attributes with values")]
pub struct AttrValue {
    #[graphql(description = "Attribute id")]
    pub attr_id: i32,
    #[graphql(description = "Attribute value")]
    pub value: String,
    #[graphql(description = "Attribute type")]
    pub value_type: AttributeType,
    #[graphql(description = "Meta field")]
    pub meta_field: Option<String>,
}

#[derive(GraphQLEnum, Deserialize, Serialize, Clone, Debug)]
#[graphql(name = "AttributeType", description = "Attribute Type")]
pub enum AttributeType {
    #[graphql(description = "String type. Can represent enums, bool, int and strings.")]
    Str,
    #[graphql(description = "Float type.")]
    Float,
}

#[derive(Serialize, Clone, Debug)]
pub struct AttributeFilter {
    pub id: i32,
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
            id: attr.id,
            filter: filter,
        })
    }
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "Attribute Filter")]
pub struct AttributeFilterInput {
    #[graphql(description = "Attribute id")]
    pub id: i32,
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
