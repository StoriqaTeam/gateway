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
#[graphql(description = "Order Status Delivery input.")]
pub struct OrderStatusDeliveryInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of order.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "Track id.")]
    pub track_id: String,
    #[graphql(description = "Comments")]
    pub comments: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct OrderStatusDelivery {
    pub status: OrderStatus,
    pub track_id: String,
    pub comments: String,
}

impl From<OrderStatusDeliveryInput> for OrderStatusDelivery {
    fn from(order: OrderStatusDeliveryInput) -> Self {
        Self {
            status: OrderStatus::Delivery,
            track_id: order.track_id,
            comments: order.comments,
        }
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Order Status Paid input.")]
pub struct OrderStatusPaidInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of order.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "Comments")]
    pub comments: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct OrderStatusPaid {
    pub status: OrderStatus,
    pub comments: String,
}

impl From<OrderStatusPaidInput> for OrderStatusPaid {
    fn from(order: OrderStatusPaidInput) -> Self {
        Self {
            status: OrderStatus::Paid,
            comments: order.comments,
        }
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Order Status Finished input.")]
pub struct OrderStatusFinishedInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Id of order.")]
    #[serde(skip_serializing)]
    pub id: GraphqlID,
    #[graphql(description = "Comments")]
    pub comments: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct OrderStatusFinished {
    pub status: OrderStatus,
    pub comments: String,
}

impl From<OrderStatusFinishedInput> for OrderStatusFinished {
    fn from(order: OrderStatusFinishedInput) -> Self {
        Self {
            status: OrderStatus::Finished,
            comments: order.comments,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct OrderHistoryItem {
    pub status: OrderStatus,
    pub user_id: i32,
    pub comments: Option<String>,
    pub creation_time: String,
}
