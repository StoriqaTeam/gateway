use std::collections::BTreeMap;

use stq_static_resources::Translation;

use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrdersCartProduct {
    pub product_id: i32,
    pub quantity: i32,
    pub store_id: i32,
    pub selected: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrdersCartItemInfo {
    pub quantity: i32,
    pub selected: bool,
    pub store_id: i32,
}

pub type CartHash = BTreeMap<i32, OrdersCartItemInfo>;

/// Base unit of user's product selection
#[derive(Deserialize, Debug, Clone)]
pub struct CartProduct {
    pub id: i32,
    pub name: Vec<Translation>,
    pub price: f64,
    pub photo_main: Option<String>,
    pub selected: bool,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cart {
    pub inner: Vec<OrdersCartProduct>,
}

impl Cart {
    pub fn new(inner: Vec<OrdersCartProduct>) -> Self {
        Self { inner }
    }
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[graphql(description = "Increment product quantity in cart input object")]
pub struct IncrementInCartInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Product id.")]
    #[serde(skip_serializing)]
    pub product_id: i32,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[graphql(description = "Set product data in cart input object")]
pub struct SetQuantityInCartInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Product id.")]
    #[serde(skip_serializing)]
    pub product_id: i32,
    #[graphql(description = "Product quantity.")]
    pub value: i32,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[graphql(description = "Set selected product data in cart input object")]
pub struct SetSelectionInCartInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Product id.")]
    #[serde(skip_serializing)]
    pub product_id: i32,
    #[graphql(description = "Product selected.")]
    pub value: bool,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[graphql(description = "Delete product from cart input object")]
pub struct DeleteFromCartInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Product id.")]
    pub product_id: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CartStore {
    pub id: i32,
    pub name: Vec<Translation>,
    pub logo: Option<String>,
    pub cover: Option<String>,
    pub rating: f64,
    pub products: Vec<CartProduct>,
}

impl CartStore {
    pub fn new(store: Store, products: Vec<CartProduct>) -> Self {
        Self {
            id: store.id,
            name: store.name,
            rating: store.rating,
            logo: store.logo,
            cover: store.cover,
            products,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartProductStore {
    pub product_id: i32,
    pub store_id: i32,
}

impl CartProductStore {
    pub fn new(product_id: i32, store_id: i32) -> Self {
        Self { product_id, store_id }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CartProductIncrementPayload {
    pub store_id: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CartMergePayload {
    pub user_from: i32,
}

pub type CartProductWithPriceHash = BTreeMap<i32, f64>;
