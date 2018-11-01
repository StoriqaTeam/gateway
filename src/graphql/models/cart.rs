use std::collections::{BTreeMap, HashMap};

use stq_static_resources::Translation;
use stq_types::{
    BaseProductId, CartItem, CompanyPackageId, CouponId, DeliveryMethodId, ProductId, ProductPrice, ProductSellerPrice, Quantity, StoreId,
    UserId,
};

use super::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrdersCartItemInfo {
    pub quantity: Quantity,
    pub selected: bool,
    pub store_id: StoreId,
    pub comment: String,
}

pub type CartHash = BTreeMap<ProductId, OrdersCartItemInfo>;

/// Base unit of user's product selection
#[derive(Deserialize, Debug, Clone)]
pub struct CartProduct {
    pub id: ProductId,
    pub name: Vec<Translation>,
    pub price: ProductPrice,
    pub photo_main: Option<String>,
    pub selected: bool,
    pub quantity: Quantity,
    pub comment: String,
    pub base_product_id: BaseProductId,
    pub pre_order: bool,
    pub pre_order_days: i32,
    pub coupon_id: Option<CouponId>,
    pub company_package_id: Option<CompanyPackageId>,
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
    #[graphql(description = "Product quantity.")]
    pub value: Option<i32>,
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
#[graphql(description = "Set coupon in cart input object")]
pub struct SetCouponInCartInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Coupon code.")]
    pub coupon_code: String,
    #[graphql(description = "Store raw id for which add coupon.")]
    pub store_id: i32,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[graphql(description = "Set delivery method in cart input object")]
pub struct SetDeliveryMethodInCartInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Product raw id.")]
    pub product_id: i32,
    #[graphql(description = "Company package id.")]
    pub company_package_id: i32,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[graphql(description = "Remove delivery method from cart input object")]
pub struct RemoveDeliveryMethodFromCartInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Product raw id.")]
    pub product_id: i32,
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
    pub product_id: ProductId,
    pub store_id: StoreId,
}

impl CartProductStore {
    pub fn new(product_id: ProductId, store_id: StoreId) -> Self {
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

pub type CartProductWithPriceHash = HashMap<ProductId, ProductSellerPrice>;

pub fn convert_to_cart(stores: Vec<Store>, products: &[CartItem]) -> Cart {
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
                                        let (quantity, selected, comment, coupon_id, company_package_id) = products
                                            .iter()
                                            .find(|v| v.product_id == variant.id)
                                            .map(|v| {
                                                let company_package_id = match v.delivery_method_id {
                                                    Some(DeliveryMethodId::Package { id }) => Some(id),
                                                    _ => None,
                                                };
                                                (v.quantity, v.selected, v.comment.clone(), v.coupon_id, company_package_id)
                                            }).unwrap_or_default();

                                        let price = if let Some(discount) = variant.discount {
                                            variant.price.0 * (1.0 - discount)
                                        } else {
                                            variant.price.0
                                        };

                                        CartProduct {
                                            id: variant.id,
                                            name: base_product.name.clone(),
                                            base_product_id: base_product.id,
                                            photo_main: variant.photo_main.clone(),
                                            selected,
                                            price: ProductPrice(price),
                                            quantity,
                                            comment,
                                            pre_order: variant.pre_order,
                                            pre_order_days: variant.pre_order_days,
                                            coupon_id,
                                            company_package_id,
                                        }
                                    }).collect::<Vec<CartProduct>>(),
                            )
                        }).unwrap_or_default()
                }).collect();
            CartStore::new(store, products)
        }).collect();
    Cart::new(cart_stores)
}
