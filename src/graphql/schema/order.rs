//! File containing PageInfo object of graphql schema
use std::cmp;
use std::collections::HashMap;
use std::str::FromStr;

use futures::Future;
use hyper::Method;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};
use serde_json;

use stq_api::{
    orders::{CartClient, Order, OrderClient},
    types::ApiFutureExt,
};
use stq_routes::{model::Model, service::Service};
use stq_static_resources::{Currency, OrderState};
use stq_types::{CouponId, OrderIdentifier, ProductSellerPrice};

use super::*;
use errors::into_graphql;
use graphql::context::Context;
use graphql::models::*;
use graphql::schema::base_product as base_product_module;
use graphql::schema::cart as cart_module;
use graphql::schema::coupon::try_get_coupon;
use graphql::schema::coupon::*;
use graphql::schema::product as product_module;
use graphql::schema::user::get_user_by_id;

graphql_object!(GraphQLOrder: Context as "Order" |&self| {
    description: "Order info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Unique id"{
        self.0.id.to_string().into()
    }

    field state() -> &OrderState as "Order State"{
        &self.0.state
    }

    field customer_id() -> &i32 as "Customer int id"{
        &self.0.customer.0
    }

    field customer(&executor) -> FieldResult<Option<User>> as "Customer" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            self.0.customer);

        context.request::<Option<User>>(Method::Get, url, None)
            .wait()
    }

    field product_id() -> &i32 as "Product int id"{
        &self.0.product.0
    }

    field deprecated "use current_product" product(&executor) -> FieldResult<Option<Product>> as "Product" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.0.product);

        context.request::<Option<Product>>(Method::Get, url, None)
            .wait()
    }

    field current_product(&executor) -> FieldResult<Option<OrderProduct>> as "Product from order." {
        let context = executor.context();
        product_module::try_get_product_without_filters(context, self.0.product).map(|product| product.map(OrderProduct))
    }

    field store_id() -> &i32 as "Store int id"{
        &self.0.store.0
    }

    field store(&executor) -> FieldResult<Option<Store>> as "Store" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            self.0.store);

        context.request::<Option<Store>>(Method::Get, url, None)
            .wait()
    }

    field quantity() -> &i32 as "Quantity" {
        &self.0.quantity.0
    }

    field price() -> &f64 as "Price" {
        &self.0.price.0
    }

    field currency() -> &Currency as "Currency" {
        &self.0.currency
    }

    field subtotal() -> f64 as "Subtotal" {
        self.0.price.0 * f64::from(self.0.quantity.0)
    }

    field coupon(&executor) -> FieldResult<Option<Coupon>> as "Coupon added user" {
        let context = executor.context();

        if let Some(coupon_id) = self.0.coupon_id {
            try_get_coupon(context, coupon_id)
        } else {
            Ok(None)
        }
    }

    field coupon_percent() -> &Option<i32> as "Coupon percent" {
        &self.0.coupon_percent
    }

    field coupon_discount() -> Option<f64> as "Coupon discount" {
        self.0.coupon_discount.map(|c| c.0)
    }

    field product_discount() -> Option<f64> as "Product discount" {
        self.0.product_discount.map(|c| c.0)
    }

    field total_amount() -> f64 as "Total amount" {
        self.0.total_amount.0
    }

    field slug() -> &i32 as "Slug" {
        &self.0.slug.0
    }

    field payment_status() -> &bool as "Payment status" {
        &self.0.payment_status
    }

    field delivery_company() -> &Option<String> as "Delivery Company" {
        &self.0.delivery_company
    }

    field delivery_price() -> &f64 as "Delivery price" {
        &self.0.delivery_price
    }

    field deprecated "use deliveryCompany and deliveryPrice" company_package_id() -> Option<i32> as "Selected package raw id" {
        self.0.company_package_id.map(|v| v.0)
    }

    field deprecated "use deliveryCompany and deliveryPrice"
    select_package(&executor) -> FieldResult<Option<AvailablePackageForUser>> as "Selected package" {
        let context = executor.context();

        match self.0.shipping_id {
            Some(shipping_id) => Ok(Some(available_packages::get_available_package_for_user_by_id_v1(context, shipping_id)?)),
            _ => Ok(None),
        }
    }

    field track_id() -> &Option<String> as "Delivery Company" {
        &self.0.track_id
    }

    field created_at() -> String as "Creation time" {
        self.0.created_at.to_rfc3339()
    }

    field receiver_name() -> &str as "Receiver name" {
        &self.0.receiver_name
    }

    field receiver_phone() -> &str as "Receiver phone" {
        &self.0.receiver_phone
    }

    field receiver_email() -> &str as "Receiver email" {
        &self.0.receiver_email
    }

    field address_full() -> Address as "Full address" {
        self.0.address.clone().into()
    }

    field pre_order() -> &bool as "Pre order" {
        &self.0.pre_order
    }

    field pre_order_days() -> &i32 as "Pre order days" {
        &self.0.pre_order_days
    }

    field history(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Offset form beginning")
            -> FieldResult<Option<Connection<OrderHistoryItem, PageInfo>>> as "History" {

        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let rpc_client = context.get_rest_api_client(Service::Orders);
        rpc_client.get_order_diff(self.0.slug.into())
            .sync()
            .map_err(into_graphql)
            .map (|items| {
                let mut item_edges: Vec<Edge<OrderHistoryItem>> = items
                    .into_iter()
                    .skip(offset as usize)
                    .take(count as usize)
                    .enumerate()
                    .map(|(i, item)| Edge::new(juniper::ID::from((i as i32 + offset).to_string()), OrderHistoryItem(item)))
                    .collect();
                let has_next_page = item_edges.len() as i32 == count + 1;
                if has_next_page {
                    item_edges.pop();
                };
                let has_previous_page = true;
                let start_cursor =  item_edges.get(0).map(|e| e.cursor.clone());
                let end_cursor = item_edges.iter().last().map(|e| e.cursor.clone());
                let page_info = PageInfo {
                    has_next_page,
                    has_previous_page,
                    start_cursor,
                    end_cursor};
                Connection::new(item_edges, page_info)
            })
            .map(Some)
    }

    field allowed_statuses(&executor) -> FieldResult<Vec<OrderState>> as "Allowed statuses" {
        let context = executor.context();
        let url = format!("{}/{}/{}/allowed_statuses",
            context.config.service_url(Service::Orders),
            Model::Order.to_url(),
            self.0.id);

        context.request::<Vec<OrderState>>(Method::Get, url, None)
            .wait()
    }

    field invoice(&executor) -> FieldResult<Option<Invoice>> as "Invoice" {
        let context = executor.context();
        let url = format!("{}/invoices/by-order-id/{}",
            context.config.service_url(Service::Billing),
            self.0.id);

        context.request::<Option<Invoice>>(Method::Get, url, None)
            .wait()
    }
});

graphql_object!(CreateOrdersOutput: Context as "CreateOrdersOutput" |&self| {
    description:"Create orders object"

    field invoice() -> &Invoice {
        &self.0
    }

    field deprecated "use cartV2" cart(&executor) -> FieldResult<Option<Cart>> as "Fetches cart products." {
        let context = executor.context();

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let fut = if let Some(session_id) = context.session_id {
            if let Some(ref user) = context.user {
                rpc_client.merge(session_id.into(), user.user_id.into())
            } else {
                rpc_client.get_cart(session_id.into())
            }
        } else if let Some(ref user) = context.user {
            rpc_client.get_cart(user.user_id.into())
        }  else {
            return Err(FieldError::new(
                "Could not get users cart.",
                graphql_value!({ "code": 100, "details": { "No user id or session id in request header." }}),
            ));
        };

        let products: Vec<_> = fut
            .sync()
            .map_err(into_graphql)?.into_iter().collect();

        cart_module::convert_products_to_cart(context, &products, None).map(Some)
    }

    field cart_v2(&executor, user_country_code: String) -> FieldResult<Option<Cart>> as "Fetches cart products." {
        let context = executor.context();

        let rpc_client = context.get_rest_api_client(Service::Orders);
        let fut = if let Some(session_id) = context.session_id {
            if let Some(ref user) = context.user {
                rpc_client.merge(session_id.into(), user.user_id.into())
            } else {
                rpc_client.get_cart(session_id.into())
            }
        } else if let Some(ref user) = context.user {
            rpc_client.get_cart(user.user_id.into())
        }  else {
            return Err(FieldError::new(
                "Could not get users cart.",
                graphql_value!({ "code": 100, "details": { "No user id or session id in request header." }}),
            ));
        };

        let products: Vec<_> = fut
            .sync()
            .map_err(into_graphql)?.into_iter().collect();

        cart_module::convert_products_to_cart(context, &products, Some(user_country_code)).map(Some)
    }
});

graphql_object!(Connection<GraphQLOrder, PageInfo>: Context as "OrdersConnection" |&self| {
    description:"Order Connection"

    field edges() -> &[Edge<GraphQLOrder>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<GraphQLOrder>: Context as "OrdersEdge" |&self| {
    description:"Order Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &GraphQLOrder {
        &self.node
    }
});

graphql_object!(Connection<OrderHistoryItem, PageInfo>: Context as "OrderHistoryItemsConnection" |&self| {
    description:"Order History Item Connection"

    field edges() -> &[Edge<OrderHistoryItem>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<OrderHistoryItem>: Context as "OrderHistoryItemsEdge" |&self| {
    description:"Order History Item Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &OrderHistoryItem {
        &self.node
    }
});

graphql_object!(OrderProduct: Context as "OrderProduct" |&self| {
    description: "Order product's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::OrderProduct, self.0.id.0).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.0.id.0
    }

    field base_product_id() -> &i32 as "Base product raw id" {
        &self.0.base_product_id.0
    }

    field is_active() -> &bool as "If the product was disabled (deleted), isActive is false" {
        &self.0.is_active
    }

    field discount() -> &Option<f64> as "Discount" {
        &self.0.discount
    }

    field currency() -> Currency as "Currency" {
        self.0.currency
    }

    field photo_main() -> &Option<String> as "Photo main" {
        &self.0.photo_main
    }

    field additional_photos() -> &Option<Vec<String>> as "Additional photos of the product." {
        &self.0.additional_photos
    }

    field vendor_code() -> &String as "Vendor code" {
        &self.0.vendor_code
    }

    field cashback() -> &Option<f64> as "Cashback" {
        &self.0.cashback
    }

    field price() -> &f64 as "Price" {
        &self.0.price.0
    }

    field pre_order() -> &bool as "Pre-order" {
        &self.0.pre_order
    }

    field pre_order_days() -> &i32 as "Pre-order days" {
        &self.0.pre_order_days
    }

    field customer_price() -> &CustomerPrice as "Customer price" {
        &self.0.customer_price
    }

    field base_product(&executor,
    ) -> FieldResult<Option<BaseProduct>> as "Fetches base product by product." {
        let context = executor.context();

        base_product_module::try_get_base_product_without_filters(context, self.0.base_product_id)
    }

    field attributes(&executor) -> FieldResult<Option<Vec<ProdAttrValue>>> as "Variants" {
       let context = executor.context();
        let url = format!("{}/{}/{}/attributes",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.0.id);

        context.request::<Vec<ProdAttrValue>>(Method::Get, url, None)
            .wait()
            .or_else(|_| Ok(vec![]))
            .map(Some)
    }

    field quantity(&executor) -> FieldResult<Option<i32>> as "Fetches product quantity from warehouses." {
        let context = executor.context();

        self.0.get_quantity(context)
    }

    field stocks(&executor,
        visibility: Option<Visibility> as "Specifies allowed visibility of the stocks") -> FieldResult<Vec<GraphQLStock>> as "Find product on warehouses." {

       let context = executor.context();
       self.0.get_stocks(context, visibility)
    }

});

graphql_object!(Connection<OrderProduct, PageInfo>: Context as "OrderProductsConnection" |&self| {
    description:"Order products Connection"

    field edges() -> &[Edge<OrderProduct>] {
        &self.edges
    }

    field page_info() -> &PageInfo {
        &self.page_info
    }
});

graphql_object!(Edge<OrderProduct>: Context as "OrderProductsEdge" |&self| {
    description:"Order products Edge"

    field cursor() -> &juniper::ID {
        &self.cursor
    }

    field node() -> &OrderProduct {
        &self.node
    }
});

pub fn run_create_orders_mutation_v1(context: &Context, input: CreateOrderInput) -> FieldResult<CreateOrdersOutput> {
    let input = input.fill_uuid();
    let user = context.user.clone().ok_or_else(|| {
        FieldError::new(
            "Could not create orders for unauthorized user.",
            graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
        )
    })?;

    let rpc_client = context.get_rest_api_client(Service::Orders);
    let current_cart = rpc_client.get_cart(user.user_id.into()).sync().map_err(into_graphql)?;

    if let Some(cart_item) = current_cart.iter().find(|p| p.delivery_method_id.is_none()) {
        return Err(FieldError::new(
            "Not select delivery package.",
            graphql_value!({ "code": 100, "details": { format!("For the product with id: {} in store: {} not set delivery package", cart_item.product_id, cart_item.store_id) }}),
        ));
    }

    let mut coupons_info = vec![];
    for cart_item in current_cart.iter() {
        if let Some(coupon_id) = cart_item.coupon_id {
            validate_coupon(context, coupon_id)?;
            let coupon = get_coupon(context, coupon_id)?;
            coupons_info.push(coupon);
        }
    }

    let products_with_prices = current_cart
        .iter()
        .map(|p| product_module::get_seller_price(context, p.product_id).and_then(|seller_price| Ok((p.product_id, seller_price))))
        .collect::<FieldResult<CartProductWithPriceHash>>()?;

    if products_with_prices.len() == 0 {
        return Err(FieldError::new(
            "Could not create orders for empty cart.",
            graphql_value!({ "code": 100, "details": { "There is no products, selected in cart." }}),
        ));
    }

    let coupons_info = coupons_info
        .into_iter()
        .map(|coupon| (coupon.id, coupon))
        .collect::<HashMap<CouponId, Coupon>>();

    let selected_packages = cart_product::get_selected_packages(context, &current_cart, None)?;

    // validate packages
    for (item, package) in selected_packages.iter() {
        cart_product::validate_select_package(item, package)?;
    }

    let packages = selected_packages
        .into_iter()
        .map(|(item, value)| (item.product_id, value))
        .collect();
    let delivery_info = cart_product::get_delivery_info(packages);
    let customer = get_user_by_id(context, user.user_id)?;

    let product_info = cart_product::get_product_info(context, &current_cart)?;

    let create_order = CreateOrder {
        customer_id: user.user_id,
        address: input.address_full,
        receiver_name: input.receiver_name,
        receiver_phone: input.receiver_phone,
        receiver_email: customer.email,
        prices: products_with_prices,
        currency: input.currency,
        coupons: coupons_info,
        delivery_info,
        product_info,
        uuid: input.uuid,
    };

    if create_order.currency.is_fiat() {
        validate_products_fiat(create_order.prices.values())?;
    }

    let url = format!("{}/create_order", context.config.saga_microservice.url.clone());
    let body: String = serde_json::to_string(&create_order)?.to_string();
    context
        .request::<Invoice>(Method::Post, url, Some(body))
        .wait()
        .map(CreateOrdersOutput)
}

pub fn run_create_orders_mutation(context: &Context, input: CreateOrderInputV2) -> FieldResult<CreateOrdersOutput> {
    let input = input.fill_uuid();
    let user = context.user.clone().ok_or_else(|| {
        FieldError::new(
            "Could not create orders for unauthorized user.",
            graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
        )
    })?;

    let rpc_client = context.get_rest_api_client(Service::Orders);
    let current_cart = rpc_client.get_cart(user.user_id.into()).sync().map_err(into_graphql)?;

    if let Some(cart_item) = current_cart.iter().find(|p| p.delivery_method_id.is_none()) {
        return Err(FieldError::new(
            "Not select delivery package.",
            graphql_value!({ "code": 100, "details": { format!("For the product with id: {} in store: {} not set delivery package", cart_item.product_id, cart_item.store_id) }}),
        ));
    }

    let mut coupons_info = vec![];
    for cart_item in current_cart.iter() {
        if let Some(coupon_id) = cart_item.coupon_id {
            validate_coupon(context, coupon_id)?;
            let coupon = get_coupon(context, coupon_id)?;
            coupons_info.push(coupon);
        }
    }

    let products_with_prices = current_cart
        .iter()
        .map(|p| product_module::get_seller_price(context, p.product_id).and_then(|seller_price| Ok((p.product_id, seller_price))))
        .collect::<FieldResult<CartProductWithPriceHash>>()?;

    if products_with_prices.len() == 0 {
        return Err(FieldError::new(
            "Could not create orders for empty cart.",
            graphql_value!({ "code": 100, "details": { "There is no products, selected in cart." }}),
        ));
    }

    let coupons_info = coupons_info
        .into_iter()
        .map(|coupon| (coupon.id, coupon))
        .collect::<HashMap<CouponId, Coupon>>();

    let selected_packages = cart_product::get_selected_packages(context, &current_cart, Some(input.user_country_code))?;

    // validate packages
    for (item, package) in selected_packages.iter() {
        cart_product::validate_select_package(item, package)?;
    }

    let packages = selected_packages
        .into_iter()
        .map(|(item, value)| (item.product_id, value))
        .collect();
    let delivery_info = cart_product::get_delivery_info(packages);
    let customer = get_user_by_id(context, user.user_id)?;

    let product_info = cart_product::get_product_info(context, &current_cart)?;

    let create_order = CreateOrder {
        customer_id: user.user_id,
        address: input.address_full,
        receiver_name: input.receiver_name,
        receiver_phone: input.receiver_phone,
        receiver_email: customer.email,
        prices: products_with_prices,
        currency: input.currency,
        coupons: coupons_info,
        delivery_info,
        product_info,
        uuid: input.uuid,
    };

    if create_order.currency.is_fiat() {
        validate_products_fiat(create_order.prices.values())?;
    }

    let url = format!("{}/create_order", context.config.saga_microservice.url.clone());

    let body: String = serde_json::to_string(&create_order)?.to_string();
    context
        .request::<Invoice>(Method::Post, url, Some(body))
        .wait()
        .map(CreateOrdersOutput)
}

pub fn try_get_order(context: &Context, order_id: OrderIdentifier) -> FieldResult<Option<GraphQLOrder>> {
    let order_route = match order_id {
        OrderIdentifier::Id(id) => format!("by-id/{}", id),
        OrderIdentifier::Slug(slug) => format!("by-slug/{}", slug),
    };

    let url = format!(
        "{}/{}/{}",
        context.config.service_url(Service::Orders),
        Model::Order.to_url(),
        order_route
    );

    context
        .request::<Option<Order>>(Method::Get, url, None)
        .wait()
        .map(|res| res.map(GraphQLOrder))
}

pub fn get_order(context: &Context, order_id: OrderIdentifier) -> FieldResult<GraphQLOrder> {
    try_get_order(context, order_id)?.ok_or_else(move || {
        let message = match order_id {
            OrderIdentifier::Id(id) => format!("by id: {}", id),
            OrderIdentifier::Slug(slug) => format!("by slug: {}", slug),
        };

        FieldError::new(
            "Order not found",
            graphql_value!({ "code": 400, "details": { format!("order {} not found", message) }}),
        )
    })
}

pub fn validate_products_fiat<'a>(products: impl Iterator<Item = &'a ProductSellerPrice>) -> FieldResult<()> {
    let mut currencies = products.map(|p| p.currency);

    if let Some(currency) = currencies.next() {
        if !currency.is_fiat() {
            return Err(FieldError::new(
                "Cart product currency is not valid.",
                graphql_value!({ "code": 100, "details": { "Cart product currency is not FIAT" }}),
            ));
        }

        for cur in currencies {
            if cur != currency {
                return Err(FieldError::new(
                    "Cart product currencies are not equal.",
                    graphql_value!({ "code": 100, "details": { "Cart contains products in different currencies" }}),
                ));
            }
        }
    }

    Ok(())
}

pub fn run_confirm_order_mutation(context: &Context, input: OrderConfirmedInput) -> FieldResult<Option<GraphQLOrder>> {
    let saga = context.get_saga_microservice();
    saga.set_order_confirmed(input.into())
}
