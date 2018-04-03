//! EAV model attributes
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};
use stq_static_resources::{Translation, TranslationInput};

#[derive(Deserialize, Debug, Clone)]
pub struct Attribute {
    pub id: i32,
    pub name: Vec<Translation>,
    pub value_type: AttributeType,
    pub meta_field: Option<AttributeMetaField>,
}

#[derive(GraphQLObject, Deserialize, Debug, Clone, PartialEq)]
#[graphql(description = "Attribute meta field")]
pub struct AttributeMetaField {
    #[graphql(description = "Possible values of attribute")]
    pub values: Option<Vec<String>>,
    #[graphql(description = "Possible values of attribute with translation")]
    pub translated_values: Option<Vec<Vec<Translation>>>,
    #[graphql(description = "UI element type ")]
    pub ui_element: Option<UIType>,
}

#[derive(GraphQLEnum, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[graphql(name = "UIType", description = "UI element type")]
pub enum UIType {
    Combobox,
    Radiobutton,
    Checkbox,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Attribute meta field input object")]
pub struct AttributeMetaFieldInput {
    #[graphql(description = "Possible values of attribute")]
    pub values: Option<Vec<String>>,
    #[graphql(description = "Possible values of attribute with translation")]
    pub translated_values: Option<Vec<Vec<TranslationInput>>>,
    #[graphql(description = "UI element type ")]
    pub ui_element: Option<UIType>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
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
    pub meta_field: Option<AttributeMetaFieldInput>,
}

impl UpdateAttributeInput {
    pub fn is_none(&self) -> bool {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            id: self.id.clone(),
            name: None,
            meta_field: None,
        } == self.clone()
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create attribute input object")]
pub struct CreateAttributeInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Name of an attribute.")]
    pub name: Vec<TranslationInput>,
    #[graphql(description = "Attribute type")]
    pub value_type: AttributeType,
    #[graphql(description = "Meta field of an attribute.")]
    pub meta_field: Option<AttributeMetaFieldInput>,
}

impl CreateAttributeInput {
    pub fn validate(&self) -> FieldResult<()> {
        if self.value_type == AttributeType::Str {
            if let Some(meta) = self.meta_field.clone() {
                if let Some(vals) = meta.values.clone() {
                    if vals.is_empty() {
                        return Err(FieldError::new(
                            "Parsing attributes meta_field error",
                            graphql_value!({ "code": 300, "details": { "There must be values variants in attribute meta_field values." }}),
                        ));
                    }
                } else if let Some(tr_vals) = meta.translated_values.clone() {
                    if tr_vals.is_empty() {
                        return Err(FieldError::new(
                            "Parsing attributes meta_field error",
                            graphql_value!({ "code": 300, "details": { "There must be values variants in attribute meta_field translated values." }}),
                        ));
                    }
                } else {
                    return Err(FieldError::new(
                        "Parsing attributes meta_field error",
                        graphql_value!({ "code": 300, "details": { "There must be values variants in attribute meta_field." }}),
                    ));
                }
            } else {
                return Err(FieldError::new(
                    "Parsing attributes meta_field error",
                    graphql_value!({ "code": 300, "details": { "There must be values variants in attribute meta_field." }}),
                ));
            }
        }
        Ok(())
    }
}

#[derive(GraphQLInputObject, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[graphql(name = "AttrValueInput", description = "Product attributes with values input object")]
pub struct AttrValueInput {
    #[graphql(description = "Int Attribute id")]
    pub attr_id: i32,
    #[graphql(description = "Attribute value")]
    pub value: String,
    #[graphql(description = "Meta field")]
    pub meta_field: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AttrValue {
    pub attr_id: i32,
    pub value: String,
    pub meta_field: Option<String>,
}

#[derive(GraphQLEnum, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[graphql(name = "AttributeType", description = "Attribute Type")]
pub enum AttributeType {
    #[graphql(description = "String type. Can represent enums, bool, int and strings.")]
    Str,
    #[graphql(description = "Float type.")]
    Float,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "Attribute Filter")]
pub struct AttributeFilterInput {
    #[graphql(description = "Int Attribute id")]
    pub id: i32,
    #[graphql(description = "Values to be equal")]
    pub equal: Option<EqualFilterInput>,
    #[graphql(description = "Range values to compare")]
    pub range: Option<RangeFilterInput>,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "Equality Filter input")]
pub struct EqualFilterInput {
    #[graphql(description = "Values to be equal")]
    pub values: Vec<String>,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Clone, Debug)]
#[graphql(description = "Range Filter input")]
pub struct RangeFilterInput {
    #[graphql(description = "Min value")]
    pub min_value: Option<f64>,
    #[graphql(description = "Max value")]
    pub max_value: Option<f64>,
}
