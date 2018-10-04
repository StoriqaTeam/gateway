use stq_types::{BaseProductId, ProductId};

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create custom attribute input object")]
pub struct NewCustomAttributeInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Attribute id.")]
    pub attribute_id: i32,
    #[graphql(description = "Base product id.")]
    pub base_product_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CustomAttribute {
    pub id: i32,
    pub attribute_id: i32,
    pub base_product_id: BaseProductId,
}

#[derive(GraphQLInputObject, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Custom attribute value input object")]
pub struct CustomAttributeValueInput {
    #[graphql(description = "Custom attribute id.")]
    pub custom_attribute_id: i32,
    #[graphql(description = "Value.")]
    pub value: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CustomAttributeValue {
    pub product_id: ProductId,
    pub custom_attribute_id: i32,
    pub value: String,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Delete custom attribute input object")]
pub struct DeleteCustomAttributeInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Attribute id.")]
    pub custom_attribute_id: i32,
}
