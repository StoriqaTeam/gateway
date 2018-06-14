#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WarehouseProduct {
    pub id: i32,
    pub warehouse_id: i32,
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Create warehouse product input object")]
pub struct CreateWarehouseProductInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Warehouse id.")]
    pub warehouse_id: i32,
    #[graphql(description = "Product id.")]
    pub product_id: i32,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Update warehouse product input object")]
pub struct UpdateWarehouseProductInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Warehouse id.")]
    #[serde(skip_serializing)]
    pub warehouse_id: i32,
    #[graphql(description = "Product id.")]
    #[serde(skip_serializing)]
    pub product_id: i32,
    #[graphql(description = "Quantity.")]
    pub quantity: i32,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "Delete warehouse product input object")]
pub struct DeleteWarehouseProductInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Warehouse id.")]
    pub warehouse_id: i32,
    #[graphql(description = "Product id.")]
    pub product_id: i32,
}
