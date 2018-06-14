use juniper::ID as GraphqlID;

use super::*;

#[derive(GraphQLEnum, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[graphql(name = "OrderStatus", description = "Current order status")]
pub enum OrderStatus {
    New,
    Paid,
    Delivery,
    Finished,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Order {
    pub id: i32,
    pub status: OrderStatus,
    pub customer_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub store_id: i32,
    pub currency_id: i32,
    pub price: f64,
    pub subtotal: f64,
    pub payment_status: bool,
    pub delivery_status: bool,
    pub delivery_company: String,
    pub delivery_track_id: Option<String>,
    pub creation_time: String,
    pub administrative_area_level_1: Option<String>,
    pub administrative_area_level_2: Option<String>,
    pub country: String,
    pub locality: Option<String>,
    pub political: Option<String>,
    pub postal_code: String,
    pub route: Option<String>,
    pub street_number: Option<String>,
    pub address: Option<String>,
    pub place_id: Option<String>,
    pub customer_comments: Option<String>,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Create order input object")]
pub struct CreateOrderInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Customer comments.")]
    pub customer_comments: Option<String>,
    #[graphql(description = "Address")]
    #[serde(flatten)]
    pub address_full: AddressInput,
    #[graphql(description = "Cart product id")]
    pub cart_product_id: i32,
    #[graphql(description = "Currency id")]
    pub currency_id: i32,
    #[graphql(description = "Price")]
    pub price: f64,
    #[graphql(description = "Subtotal")]
    pub subtotal: f64,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Update order input object")]
pub struct UpdateOrderInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of order.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "Status of order.")]
    pub status: Option<OrderStatus>,
    #[graphql(description = "Payment status.")]
    pub payment_status: Option<bool>,
    #[graphql(description = "Delivery status.")]
    pub delivery_status: Option<bool>,
}

impl UpdateOrderInput {
    pub fn is_none(&self) -> bool {
        Self {
            client_mutation_id: self.client_mutation_id.clone(),
            id: self.id.clone(),
            payment_status: None,
            delivery_status: None,
            status: None,
        } == self.clone()
    }
}
