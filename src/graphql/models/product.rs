use juniper::ID as GraphqlID;
use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub id: i32,
    pub base_product_id: i32,
    pub is_active: bool,
    pub discount: Option<f64>,
    pub photo_main: Option<String>,
    pub additional_photos: Option<Vec<String>>,
    pub vendor_code: Option<String>,
    pub cashback: Option<f64>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
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
    pub attributes: Vec<AttrValueInput>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Update product input object")]
pub struct UpdateProduct {
    #[graphql(description = "discount")]
    pub discount: Option<f64>,
    #[graphql(description = "photo_main")]
    pub photo_main: Option<String>,
    #[graphql(description = "Additional photos of the product.")]
    pub additional_photos: Option<Vec<String>>,
    #[graphql(description = "vendor code")]
    pub vendor_code: Option<String>,
    #[graphql(description = "cashback")]
    pub cashback: Option<f64>,
}

impl UpdateProductWithAttributesInput {
    pub fn is_none(&self) -> bool {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            id: self.id.clone(),
            product: UpdateProduct {
                discount: None,
                photo_main: None,
                additional_photos: None,
                vendor_code: None,
                cashback: None,
            },
            attributes: vec![],
        } == self.clone()
    }
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
    pub attributes: Vec<AttrValueInput>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone)]
#[graphql(description = "New Product")]
pub struct NewProduct {
    #[graphql(description = "Base product id variant belonging to.")]
    pub base_product_id: i32,
    #[graphql(description = "Discount.")]
    pub discount: Option<f64>,
    #[graphql(description = "Main photo of the product.")]
    pub photo_main: Option<String>,
    #[graphql(description = "Additional photos of the product.")]
    pub additional_photos: Option<Vec<String>>,
    #[graphql(description = "Vendor code.")]
    pub vendor_code: Option<String>,
    #[graphql(description = "Cashback.")]
    pub cashback: Option<f64>,
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "Deactivate product input object")]
pub struct DeactivateProductInput {
    #[graphql(description = "Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a product.")]
    pub id: GraphqlID,
}
