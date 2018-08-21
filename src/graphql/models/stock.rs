use stq_api::warehouses::Stock;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphQLStock(pub Stock);

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create warehouse product input object")]
pub struct CreateStockInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Warehouse id.")]
    pub warehouse_id: String,
    #[graphql(description = "Product id.")]
    pub product_id: i32,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Product Quantity input object")]
pub struct ProductQuantityInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Warehouse id.")]
    #[serde(skip_serializing)]
    pub warehouse_id: String,
    #[graphql(description = "Product id.")]
    #[serde(skip_serializing)]
    pub product_id: i32,
    #[graphql(description = "Quantity.")]
    pub quantity: i32,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Delete warehouse product input object")]
pub struct DeleteStockInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Warehouse id.")]
    pub warehouse_id: String,
    #[graphql(description = "Product id.")]
    pub product_id: i32,
}
