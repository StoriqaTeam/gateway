//! EAV model attributes
use serde::ser::{Error, Serialize, SerializeStruct, Serializer};
use juniper::ID as GraphqlID;
use stq_static_resources::{Translation, TranslationInput};

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Filter {
    Equal(String),
    Lte(f32),
    Gte(f32),
}

#[derive(GraphQLInputObject, Deserialize, Clone, Debug)]
#[graphql(description = "Attribute Filter")]
pub struct AttributeFilterInput {
    #[graphql(description = "Attribute id")]
    pub id: i32,
    #[graphql(description = "Attribute type")]
    pub filter_type: FilterTypeInput,
    #[graphql(description = "Attribute value")]
    pub value: String,
}

impl Serialize for AttributeFilterInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 4 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("AttributeFilter", 2)?;
        state.serialize_field("id", &self.id)?;
        let filter = match &self.filter_type {
            &FilterTypeInput::Equal => Filter::Equal(self.value.clone()),
            v => {
                let val = self.value.parse().map_err(S::Error::custom)?;
                match v {
                    &FilterTypeInput::Lte => Filter::Lte(val),
                    &FilterTypeInput::Gte => Filter::Gte(val),
                    _ => unreachable!(),
                }
            }
        };
        state.serialize_field("filter", &filter)?;
        state.end()
    }
}

#[derive(GraphQLEnum, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "Filter type. Equal can be used for strings, enums, bool, ints: value will be interpreted as string. Other filters will be applied to float values.")]
pub enum FilterTypeInput {
    #[graphql(description = "Equal")]
    Equal,
    #[graphql(description = "Less than Equal")]
    Lte,
    #[graphql(description = "Greater than Equal")]
    Gte,
}
