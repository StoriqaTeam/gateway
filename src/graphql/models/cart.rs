use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartProduct {
    pub product_id: i32,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cart {
    pub inner: Vec<CartProduct>
}

impl Cart {
    pub fn new(inner: Vec<CartProduct>) -> Self {
        Self {
            inner
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCart {
    pub products: BTreeMap<i32, i32>,
}

pub fn cart_from_orders_reply(v: OrdersCart) -> Cart {
    let inner = v.products
        .into_iter()
        .map(|(product_id, quantity)| CartProduct { product_id, quantity })
        .collect::<Vec<CartProduct>>();
    Cart::new(inner)
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[graphql(description = "Increment product quantity in cart input object")]
pub struct IncrementInCartInput {
    #[graphql(description = "Product id.")]
    #[serde(skip_serializing)]
    pub product_id: i32,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[graphql(description = "Set product data in cart input object")]
pub struct SetInCartInput {
    #[graphql(description = "Product id.")]
    #[serde(skip_serializing)]
    pub product_id: i32,
    #[graphql(description = "Product quantity.")]
    pub quantity: i32,
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[graphql(description = "Delete product from cart input object")]
pub struct DeleteFromCartInput {
    #[graphql(description = "Product id.")]
    pub product_id: i32,
}
