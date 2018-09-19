use super::*;
use juniper::ID as GraphqlID;

use stq_static_resources::Currency;
use stq_types::{BaseProductId, ProductId, ProductPrice};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub id: ProductId,
    pub base_product_id: BaseProductId,
    pub is_active: bool,
    pub discount: Option<f64>,
    pub photo_main: Option<String>,
    pub additional_photos: Option<Vec<String>>,
    pub vendor_code: String,
    pub cashback: Option<f64>,
    pub currency: Currency,
    pub price: ProductPrice,
    pub pre_order: bool,
    pub pre_order_days: i32,
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
    pub product: Option<UpdateProduct>,
    #[graphql(description = "Attributes")]
    pub attributes: Option<Vec<AttrValueInput>>,
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
    #[graphql(description = "price")]
    pub price: Option<f64>,
    #[graphql(description = "Pre-order.")]
    pub pre_order: Option<bool>,
    #[graphql(description = "Pre-order days.")]
    pub pre_order_days: Option<i32>,
}

impl UpdateProductWithAttributesInput {
    pub fn is_none(&self) -> bool {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            id: self.id.clone(),
            product: Some(UpdateProduct {
                discount: None,
                photo_main: None,
                additional_photos: None,
                vendor_code: None,
                cashback: None,
                price: None,
                pre_order: None,
                pre_order_days: None,
            }),
            attributes: Some(vec![]),
        } == self.clone()
            || Self {
                client_mutation_id: self.client_mutation_id.clone(),
                id: self.id.clone(),
                product: None,
                attributes: None,
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
    #[graphql(description = "Int Base product id variant belonging to.")]
    pub base_product_id: i32,
    #[graphql(description = "Discount.")]
    pub discount: Option<f64>,
    #[graphql(description = "Main photo of the product.")]
    pub photo_main: Option<String>,
    #[graphql(description = "Additional photos of the product.")]
    pub additional_photos: Option<Vec<String>>,
    #[graphql(description = "Vendor code.")]
    pub vendor_code: String,
    #[graphql(description = "Cashback.")]
    pub cashback: Option<f64>,
    #[graphql(description = "Price.")]
    pub price: f64,
    #[graphql(description = "Pre-order.")]
    pub pre_order: bool,
    #[graphql(description = "Pre-order days.")]
    pub pre_order_days: i32,
}

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "Deactivate product input object")]
pub struct DeactivateProductInput {
    #[graphql(description = "Client mutation id.")]
    pub client_mutation_id: String,
    #[graphql(description = "Id of a product.")]
    pub id: GraphqlID,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Variants {
    pub products: Vec<Product>,
}

impl Variants {
    pub fn new(products: Vec<Product>) -> Self {
        Self { products }
    }

    pub fn get_most_discount(&self) -> Option<&Product> {
        self.products
            .iter()
            .filter_map(|p| if p.discount.is_some() { Some(p) } else { None })
            .max_by_key(|p| (p.discount.unwrap() * 1000f64).round() as i64)
    }

    pub fn get_first(&self) -> Option<&Product> {
        self.products.get(0)
    }
}
