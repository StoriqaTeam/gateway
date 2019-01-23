use std::collections::HashMap;

use chrono::prelude::*;
use uuid::Uuid;

use stq_api::orders::{Order, OrderDiff};
use stq_static_resources::{CommitterRole, Currency, CurrencyType, OrderState};
use stq_types::{
    BaseProductId, CashbackPercent, CompanyPackageId, CouponId, OrderSlug, ProductId, ProductSellerPrice, Quantity, ShippingId, StoreId,
    UserId,
};

use super::*;

#[derive(Deserialize, Debug, Clone)]
pub struct GraphQLOrder(pub Order);

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Create order input object")]
pub struct CreateOrderInput {
    #[graphql(name = "clientMutationId", description = "Client mutation id.")]
    pub uuid: String,
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

impl CreateOrderInput {
    pub fn fill_uuid(mut self) -> Self {
        self.uuid = Some(self.uuid)
            .filter(|id| !id.is_empty())
            .unwrap_or_else(|| Uuid::new_v4().hyphenated().to_string());
        self
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Create order input object")]
pub struct CreateOrderInputV2 {
    #[graphql(name = "clientMutationId", description = "Client mutation id.")]
    pub uuid: String,
    #[graphql(description = "Address")]
    #[serde(flatten)]
    pub address_full: AddressInput,
    #[graphql(description = "Receiver name")]
    pub receiver_name: String,
    #[graphql(description = "Receiver phone")]
    pub receiver_phone: String,
    #[graphql(description = "Currency that will be paid")]
    pub currency: Currency,
    #[graphql(description = "User country code")]
    pub user_country_code: String,
}

impl CreateOrderInputV2 {
    pub fn fill_uuid(mut self) -> Self {
        self.uuid = Some(self.uuid)
            .filter(|id| !id.is_empty())
            .unwrap_or_else(|| Uuid::new_v4().hyphenated().to_string());
        self
    }
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeliveryInfo {
    pub company_package_id: CompanyPackageId, // TODO: drop this field
    pub shipping_id: ShippingId,
    pub name: String,
    pub logo: String,
    pub price: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProductInfo {
    pub base_product_id: BaseProductId,
    pub cashback: Option<CashbackPercent>,
    pub pre_order: bool,
    pub pre_order_days: i32,
}

impl From<Product> for ProductInfo {
    fn from(other: Product) -> Self {
        Self {
            base_product_id: other.base_product_id,
            cashback: other.cashback.map(CashbackPercent),
            pre_order: other.pre_order,
            pre_order_days: other.pre_order_days,
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct CreateOrder {
    pub customer_id: UserId,
    #[serde(flatten)]
    pub address: AddressInput,
    pub receiver_name: String, // TODO: move in customer_info
    pub prices: CartProductWithPriceHash,
    pub currency: Currency,
    pub receiver_phone: String, // TODO: move in customer_info
    pub receiver_email: String, // TODO: move in customer_info
    pub coupons: HashMap<CouponId, Coupon>,
    pub delivery_info: HashMap<ProductId, DeliveryInfo>,
    pub product_info: HashMap<ProductId, ProductInfo>,
    pub uuid: String,
    pub currency_type: Option<CurrencyType>,
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
    pub committer_role: CommitterRole,
}

impl From<OrderStatusDeliveryInput> for OrderStatusDelivery {
    fn from(order: OrderStatusDeliveryInput) -> Self {
        Self {
            state: OrderState::Sent,
            track_id: order.track_id,
            comment: order.comment,
            committer_role: CommitterRole::Seller,
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
    #[graphql(description = "Committer Role, by default - System.")]
    pub committer_role: Option<CommitterRole>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct OrderStatusCanceled {
    pub state: OrderState,
    pub comment: Option<String>,
    pub committer_role: CommitterRole,
}

impl From<OrderStatusCanceledInput> for OrderStatusCanceled {
    fn from(order: OrderStatusCanceledInput) -> Self {
        Self {
            state: OrderState::Cancelled,
            comment: order.comment,
            committer_role: order.committer_role.unwrap_or_else(|| CommitterRole::System),
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
    #[graphql(description = "Committer Role, by default - System.")]
    pub committer_role: Option<CommitterRole>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct OrderStatusComplete {
    pub state: OrderState,
    pub comment: Option<String>,
    pub committer_role: CommitterRole,
}

impl From<OrderStatusCompleteInput> for OrderStatusComplete {
    fn from(order: OrderStatusCompleteInput) -> Self {
        Self {
            state: OrderState::Complete,
            comment: order.comment,
            committer_role: order.committer_role.unwrap_or_else(|| CommitterRole::System),
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
    #[graphql(name = "clientMutationId", description = "Client mutation id.")]
    pub uuid: String,
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
    #[graphql(description = "Coupon code added user")]
    pub coupon_code: Option<String>,
    #[graphql(description = "Select delivery package shipping id")]
    pub shipping_id: i32,
}

impl BuyNowInput {
    pub fn fill_uuid(mut self) -> Self {
        self.uuid = Some(self.uuid)
            .filter(|id| !id.is_empty())
            .unwrap_or_else(|| Uuid::new_v4().hyphenated().to_string());
        self
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Buy now input object")]
pub struct BuyNowInputV2 {
    #[graphql(name = "clientMutationId", description = "Client mutation id.")]
    pub uuid: String,
    #[graphql(description = "Product id")]
    pub product_id: i32,
    #[graphql(description = "Quantity")]
    pub quantity: i32,
    #[graphql(description = "Address")]
    #[serde(flatten)]
    pub address_full: AddressInput,
    #[graphql(description = "Country code of the user")]
    pub user_country_code: String,
    #[graphql(description = "Receiver name")]
    pub receiver_name: String,
    #[graphql(description = "Receiver phone")]
    pub receiver_phone: String,
    #[graphql(description = "Currency that will be paid")]
    pub currency: Currency,
    #[graphql(description = "Coupon code added user")]
    pub coupon_code: Option<String>,
    #[graphql(description = "Select delivery package shipping id")]
    pub shipping_id: i32,
}

impl BuyNowInputV2 {
    pub fn fill_uuid(mut self) -> Self {
        self.uuid = Some(self.uuid)
            .filter(|id| !id.is_empty())
            .unwrap_or_else(|| Uuid::new_v4().hyphenated().to_string());
        self
    }
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
    pub receiver_email: String,
    pub pre_order: bool,
    pub pre_order_days: i32,
    pub coupon: Option<Coupon>,
    pub delivery_info: Option<DeliveryInfo>, // TODO: drop Option<T>
    pub product_info: ProductInfo,
    pub uuid: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OrderProduct(pub Product);

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Order confirm for seller input.")]
pub struct OrderConfirmedInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Slug of order.")]
    pub order_slug: i32,
    #[graphql(description = "Comment")]
    pub comment: Option<String>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct OrderConfirmed {
    pub order_slug: OrderSlug,
    pub state: OrderState,
    pub comment: Option<String>,
    pub committer_role: CommitterRole,
}

impl From<OrderConfirmedInput> for OrderConfirmed {
    fn from(order: OrderConfirmedInput) -> Self {
        Self {
            order_slug: OrderSlug(order.order_slug),
            state: OrderState::InProcessing,
            comment: order.comment,
            committer_role: CommitterRole::Seller,
        }
    }
}

#[derive(GraphQLInputObject, Serialize, Debug, Clone, PartialEq)]
#[graphql(description = "Confirmation by the financier that the money is transferred to the seller input object.")]
pub struct PaidToSellerOrderStateInput {
    #[graphql(description = "Client mutation id.")]
    #[serde(skip_serializing)]
    pub client_mutation_id: String,
    #[graphql(description = "Slug of order.")]
    pub order_id: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct OrderPaymentState {
    pub state: PaymentState,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentState {
    /// Order created and maybe paid by customer
    Initial,
    /// Store manager declined the order
    Declined,
    /// Store manager confirmed the order, money was captured
    Captured,
    /// Need money refund to customer
    RefundNeeded,
    /// Money was refunded to customer
    Refunded,
    /// Money was paid to seller
    PaidToSeller,
    /// Need money payment to seller
    PaymentToSellerNeeded,
}
