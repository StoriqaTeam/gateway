use std::collections::{BTreeMap, HashMap};

use stq_static_resources::Translation;
use stq_types::{StoreId, UserId};

use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrdersCartProduct {
    pub product_id: i32,
    pub quantity: i32,
    pub store_id: StoreId,
    pub selected: bool,
    pub comment: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrdersCartItemInfo {
    pub quantity: i32,
    pub selected: bool,
    pub store_id: StoreId,
    pub comment: String,
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
    pub comment: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Cart {
    pub inner: Vec<CartStore>,
}

impl Cart {
    pub fn new(inner: Vec<CartStore>) -> Self {
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
#[graphql(description = "Set product comment in cart input object")]
pub struct SetCommentInCartInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Product id.")]
    #[serde(skip_serializing)]
    pub product_id: i32,
    #[graphql(description = "Product comment.")]
    pub value: String,
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
    pub id: StoreId,
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
    pub store_id: StoreId,
}

impl CartProductStore {
    pub fn new(product_id: i32, store_id: StoreId) -> Self {
        Self { product_id, store_id }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CartProductIncrementPayload {
    pub store_id: StoreId,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CartMergePayload {
    pub user_from: UserId,
}

pub type CartProductWithPriceHash = HashMap<i32, f64>;

pub fn convert_to_cart(stores: Vec<Store>, products: Vec<OrdersCartProduct>) -> Cart {
    let cart_stores: Vec<CartStore> = stores
        .into_iter()
        .map(|store| {
            let products = store
                .base_products
                .clone()
                .unwrap_or_default()
                .into_iter()
                .flat_map(|base_product| {
                    base_product
                        .variants
                        .clone()
                        .and_then(|mut v| {
                            Some(
                                v.iter_mut()
                                    .map(|variant| {
                                        let (quantity, selected, comment) = products
                                            .iter()
                                            .find(|v| v.product_id == variant.id)
                                            .map(|v| (v.quantity, v.selected, v.comment.clone()))
                                            .unwrap_or_default();

                                        let price = if let Some(discount) = variant.discount {
                                            variant.price * (1.0 - discount)
                                        } else {
                                            variant.price
                                        };

                                        CartProduct {
                                            id: variant.id,
                                            name: base_product.name.clone(),
                                            photo_main: variant.photo_main.clone(),
                                            selected,
                                            price,
                                            quantity,
                                            comment,
                                        }
                                    })
                                    .collect::<Vec<CartProduct>>(),
                            )
                        })
                        .unwrap_or_default()
                })
                .collect();
            CartStore::new(store, products)
        })
        .collect();
    Cart::new(cart_stores)
}
