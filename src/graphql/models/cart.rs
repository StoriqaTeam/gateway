use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartProduct {
    pub product_id: i32,
    pub quantity: i32,
}

pub type Cart = Vec<CartProduct>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrdersCart {
    pub products: BTreeMap<i32, i32>,
}

pub fn cart_from_orders_reply(v: OrdersCart) -> Cart {
    v.products
        .into_iter()
        .map(|(product_id, quantity)| CartProduct {
            product_id,
            quantity,
        })
        .collect::<Cart>()
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
    pub product_id: Option<i32>,
}
