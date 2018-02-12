use juniper::ID as GraphqlID;

#[derive(Deserialize, Debug, Clone)]
pub struct Product {
    pub id: i32,
    pub product_id: i32,
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
#[graphql(description="Update product input object")]
pub struct UpdateProductInput {
    #[graphql(description="Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description="Id of a product.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description="New name of a product.")]
    pub name: Option<String>,
    #[graphql(description="currency_id")]
    pub currency_id: Option<i32>,
    #[graphql(description="short_description")]
    pub short_description: Option<String>,
    #[graphql(description="long_description")]
    pub long_description: Option<String>,
    #[graphql(description="price")]
    pub price: Option<f64>,
    #[graphql(description="discount")]
    pub discount: Option<f64>,
    #[graphql(description="category")]
    pub category: Option<i32>,
    #[graphql(description="photo_main")]
    pub photo_main: Option<String>,
}



#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description="Create product input object")]
pub struct CreateProductInput {
    #[graphql(description="Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description="New name of a product.")]
    pub name: String,
    pub store_id: i32,
    #[graphql(description="currency_id")]
    pub currency_id: i32,
    #[graphql(description="short_description")]
    pub short_description: String,
    #[graphql(description="long_description")]
    pub long_description: Option<String>,
    #[graphql(description="price")]
    pub price: f64,
    #[graphql(description="discount")]
    pub discount: Option<f64>,
    #[graphql(description="category")]
    pub category: Option<i32>,
    #[graphql(description="photo_main")]
    pub photo_main: Option<String>,
}


#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description="Deactivate product input object")]
pub struct DeactivateProductInput {
    #[graphql(description="Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description="Id of a product.")]
    pub id: GraphqlID,
}