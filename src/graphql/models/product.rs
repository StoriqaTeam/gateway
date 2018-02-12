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

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct UpdateProduct {
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

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description="Update product input object")]
pub struct UpdateProductInput {
    #[graphql(description="Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description="Input field update product.")]
    pub input_fields: UpdateProductWithIdInput
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description="Update product with id input object")]
pub struct UpdateProductWithIdInput {
    #[graphql(description="Id of a product.")]
    pub id: GraphqlID,
    #[graphql(description="Input field update product.")]
    pub update_product: UpdateProduct
}


#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description="New product")]
pub struct NewProduct {
    pub name: String,
    pub store_id: i32,
    pub currency_id: i32,
    pub short_description: String,
    pub long_description: Option<String>,
    pub price: f64,
    pub discount: Option<f64>,
    pub category: Option<i32>,
    pub photo_main: Option<String>,
}


#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description="Create product input object")]
pub struct CreateProductInput {
    #[graphql(description="Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description="Input field new product.")]
    pub input_fields: NewProduct
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description="Delete product")]
pub struct DeleteProduct {
    #[graphql(description="Email of a product.")]
    pub id: GraphqlID,
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description="Deactivate product input object")]
pub struct DeactivateProductInput {
    #[graphql(description="Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description="Input field Delete product.")]
    pub input_fields: DeleteProduct
}