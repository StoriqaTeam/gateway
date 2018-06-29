use chrono::prelude::*;
use juniper::ID as GraphqlID;

use super::*;

#[derive(GraphQLEnum, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[graphql(name = "OrderStatus", description = "Current order status")]
pub enum OrderStatus {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "in_processing")]
    InProcessing,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "complete")]
    Complete,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Order {
    pub id: String,
    pub state: OrderStatus,
    #[serde(rename = "customer")]
    pub customer_id: i32,
    #[serde(rename = "product")]
    pub product_id: i32,
    pub quantity: i32,
    #[serde(rename = "store")]
    pub store_id: i32,
    pub price: f64,
    pub receiver_name: String,
    pub slug: i32,
    pub payment_status: bool,
    pub delivery_company: Option<String>,
    pub track_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub address: Address,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Create order input object")]
pub struct CreateOrderInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Address")]
    #[serde(flatten)]
    pub address_full: AddressInput,
    #[graphql(description = "Receiver name")]
    pub receiver_name: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CreateOrder {
    pub customer_id: i32,
    #[serde(flatten)]
    pub address: AddressInput,
    pub receiver_name: String,
    pub prices: CartProductWithPriceHash,
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
            status: OrderStatus::Sent,
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
#[graphql(description = "Order Status Complete input.")]
pub struct OrderStatusCompleteInput {
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
pub struct OrderStatusComplete {
    pub status: OrderStatus,
    pub comments: String,
}

impl From<OrderStatusCompleteInput> for OrderStatusComplete {
    fn from(order: OrderStatusCompleteInput) -> Self {
        Self {
            status: OrderStatus::Complete,
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

#[derive(GraphQLInputObject, Serialize, Clone, Debug, Default)]
#[graphql(description = "Search order option input object")]
pub struct SearchOrderOptionInput {
    #[graphql(description = "Slug")]
    pub slug: Option<String>,
    #[graphql(description = "Customer email")]
    pub email: Option<String>,
    #[graphql(description = "Min Date")]
    pub created_from: Option<String>,
    #[graphql(description = "Max Date")]
    pub created_to: Option<String>,
    #[graphql(description = "Payment status")]
    pub payment_status: Option<bool>,
    #[graphql(description = "Order status")]
    pub order_status: Option<OrderStatus>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct SearchOrder {
    pub slug: Option<String>,
    pub customer: Option<i32>,
    pub store: Option<i32>,
    pub created_from: Option<DateTime<Utc>>,
    pub created_to: Option<DateTime<Utc>>,
    pub payment_status: Option<bool>,
    pub state: Option<OrderStatus>,
}

#[derive(Clone, Debug)]
pub struct PageInfoOrdersSearch {
    pub total_pages: i32,
    pub current_page: i32,
    pub page_items_count: i32,
    pub search_term_options: Option<SearchOrderOption>,
}

#[derive(GraphQLObject, Serialize, Clone, Debug, Default)]
#[graphql(description = "Search order option object")]
pub struct SearchOrderOption {
    #[graphql(description = "Slug")]
    pub slug: Option<String>,
    #[graphql(description = "Customer email")]
    pub email: Option<String>,
    #[graphql(description = "Min Date")]
    pub created_from: Option<String>,
    #[graphql(description = "Max Date")]
    pub created_to: Option<String>,
    #[graphql(description = "Payment status")]
    pub payment_status: Option<bool>,
    #[graphql(description = "Order status")]
    pub order_status: Option<OrderStatus>,
}

impl From<SearchOrderOptionInput> for SearchOrderOption {
    fn from(options: SearchOrderOptionInput) -> Self {
        Self {
            slug: options.slug,
            email: options.email,
            created_from: options.created_from,
            created_to: options.created_to,
            payment_status: options.payment_status,
            order_status: options.order_status,
        }
    }
}
