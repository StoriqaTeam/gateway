use std::collections::HashMap;

use chrono::prelude::*;

use stq_api::orders::{Order, OrderDiff};
use stq_static_resources::{Currency, OrderState};
use stq_types::{CouponId, OrderSlug, ProductId, ProductSellerPrice, Quantity, StoreId, UserId};

use super::*;

#[derive(Deserialize, Debug, Clone)]
pub struct GraphQLOrder(pub Order);

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
    #[graphql(description = "Receiver phone")]
    pub receiver_phone: String,
    #[graphql(description = "Currency that will be paid")]
    pub currency: Currency,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Create order paying with FIAT input object")]
pub struct CreateOrderFiatInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Address")]
    #[serde(flatten)]
    pub address_full: AddressInput,
    #[graphql(description = "Receiver name")]
    pub receiver_name: String,
    #[graphql(description = "Receiver phone")]
    pub receiver_phone: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CreateOrder {
    pub customer_id: UserId,
    #[serde(flatten)]
    pub address: AddressInput,
    pub receiver_name: String,
    pub prices: CartProductWithPriceHash,
    pub currency: Currency,
    pub receiver_phone: String,
    pub coupons: HashMap<CouponId, Coupon>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CreateOrderFiat {
    pub customer_id: UserId,
    #[serde(flatten)]
    pub address: AddressInput,
    pub receiver_name: String,
    pub prices: CartProductWithPriceHash,
    pub receiver_phone: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BillingOrders {
    pub orders: Vec<Order>,
    pub url: String,
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Order Status Delivery input.")]
pub struct OrderStatusDeliveryInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Slug of order.")]
    #[serde(skip_serializing)]
    pub order_slug: i32,
    #[graphql(description = "Track id.")]
    pub track_id: Option<String>,
    #[graphql(description = "Comment.")]
    pub comment: Option<String>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct OrderStatusDelivery {
    pub state: OrderState,
    pub track_id: Option<String>,
    pub comment: Option<String>,
}

impl From<OrderStatusDeliveryInput> for OrderStatusDelivery {
    fn from(order: OrderStatusDeliveryInput) -> Self {
        Self {
            state: OrderState::Sent,
            track_id: order.track_id,
            comment: order.comment,
        }
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Order Status Canceled input.")]
pub struct OrderStatusCanceledInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Slug of order.")]
    #[serde(skip_serializing)]
    pub order_slug: i32,
    #[graphql(description = "Comment")]
    pub comment: Option<String>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct OrderStatusCanceled {
    pub state: OrderState,
    pub comment: Option<String>,
}

impl From<OrderStatusCanceledInput> for OrderStatusCanceled {
    fn from(order: OrderStatusCanceledInput) -> Self {
        Self {
            state: OrderState::Cancelled,
            comment: order.comment,
        }
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Order Status Complete input.")]
pub struct OrderStatusCompleteInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Slug of order.")]
    #[serde(skip_serializing)]
    pub order_slug: i32,
    #[graphql(description = "Comment")]
    pub comment: Option<String>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct OrderStatusComplete {
    pub state: OrderState,
    pub comment: Option<String>,
}

impl From<OrderStatusCompleteInput> for OrderStatusComplete {
    fn from(order: OrderStatusCompleteInput) -> Self {
        Self {
            state: OrderState::Complete,
            comment: order.comment,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct OrderHistoryItem(pub OrderDiff);

#[derive(GraphQLInputObject, Serialize, Clone, Debug, Default)]
#[graphql(description = "Search order option input object")]
pub struct SearchOrderOptionInput {
    #[graphql(description = "Slug")]
    pub slug: Option<i32>,
    #[graphql(description = "Customer email")]
    pub email: Option<String>,
    #[graphql(description = "Min Date")]
    pub created_from: Option<String>,
    #[graphql(description = "Max Date")]
    pub created_to: Option<String>,
    #[graphql(description = "Payment status")]
    pub payment_status: Option<bool>,
    #[graphql(description = "Order status")]
    pub order_status: Option<OrderState>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct SearchOrder {
    pub slug: Option<OrderSlug>,
    pub customer: Option<UserId>,
    pub store: Option<StoreId>,
    pub created_from: Option<DateTime<Utc>>,
    pub created_to: Option<DateTime<Utc>>,
    pub payment_status: Option<bool>,
    pub state: Option<OrderState>,
}

#[derive(Clone, Debug)]
pub struct PageInfoOrdersSearch {
    pub total_pages: i32,
    pub current_page: i32,
    pub page_items_count: i32,
    pub search_term_options: SearchOrderOption,
}

#[derive(GraphQLObject, Serialize, Clone, Debug, Default)]
#[graphql(description = "Search order option object")]
pub struct SearchOrderOption {
    #[graphql(description = "Slug")]
    pub slug: Option<i32>,
    #[graphql(description = "Customer email")]
    pub email: Option<String>,
    #[graphql(description = "Min Date")]
    pub created_from: Option<String>,
    #[graphql(description = "Max Date")]
    pub created_to: Option<String>,
    #[graphql(description = "Payment status")]
    pub payment_status: Option<bool>,
    #[graphql(description = "Order status")]
    pub order_status: Option<OrderState>,
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

#[derive(Clone, Debug)]
pub struct CreateOrdersOutput(pub Invoice);

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Buy now input object")]
pub struct BuyNowInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Product id")]
    pub product_id: i32,
    #[graphql(description = "Quantity")]
    pub quantity: i32,
    #[graphql(description = "Address")]
    #[serde(flatten)]
    pub address_full: AddressInput,
    #[graphql(description = "Receiver name")]
    pub receiver_name: String,
    #[graphql(description = "Receiver phone")]
    pub receiver_phone: String,
    #[graphql(description = "Currency that will be paid")]
    pub currency: Currency,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct BuyNow {
    pub product_id: ProductId,
    pub customer_id: UserId,
    pub store_id: StoreId,
    pub address: AddressInput,
    pub receiver_name: String,
    pub price: ProductSellerPrice,
    pub quantity: Quantity,
    pub currency: Currency,
    pub receiver_phone: String,
    pub pre_order: bool,
    pub pre_order_days: i32,
}
