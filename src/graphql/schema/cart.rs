//! File containing Cart object of graphql schema

use std::cmp;
use std::str::FromStr;

use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use futures::Future;
use hyper::Method;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_types::{CartItem, DeliveryMethodId, ProductId, ShippingId, UserId};

use stq_api::orders::CartClient;
use stq_api::types::ApiFutureExt;

use super::*;
use graphql::context::Context;
use graphql::models::*;
use graphql::schema::cart_store::{
    calculate_coupons_discount, calculate_products_delivery_cost, calculate_products_price, calculate_products_price_without_discounts,
};

use errors::into_graphql;
use graphql::schema::available_packages;
use graphql::schema::product;

graphql_object!(Cart: Context as "Cart" |&self| {
    description: "Users cart"

    interfaces: [&Node]

    field id(&executor) -> GraphqlID as "Base64 Unique id"{
        let context = executor.context();

        if let Some(ref user) = context.user {
            ID::new(Service::Orders, Model::Cart, user.user_id.0).to_string().into()
        } else if let Some(session_id) = context.session_id {
            session_id.0.to_string().into()
        }  else {
            ID::new(Service::Orders, Model::Cart, UserId::default().0).to_string().into()
        }

    }

    field stores(&executor,
        first = None : Option<i32> as "First edges",
        after = None : Option<GraphqlID>  as "Offset")
            -> Connection<CartStore, PageInfo> as "Fetches stores using relay connection." {
        let context = executor.context();

        let offset = after
            .and_then(|id|{
                i32::from_str(&id).map(|i| i + 1).ok()
            })
            .unwrap_or_default();

        let records_limit = context.config.gateway.records_limit;
        let count = cmp::min(first.unwrap_or(records_limit as i32), records_limit as i32);

        let mut cart_stores: Vec<CartStore> = self.inner.clone()
            .into_iter()
            .skip(offset as usize)
            .take(count as usize)
            .collect();
        let mut store_edges = Edge::create_vec(cart_stores, offset);
        let has_next_page = store_edges.len() as i32 > count;
        let has_previous_page = true;
        let start_cursor =  store_edges.get(0).map(|e| e.cursor.clone());
        let end_cursor = store_edges.iter().last().map(|e| e.cursor.clone());
        let page_info = PageInfo {
            has_next_page,
            has_previous_page,
            start_cursor,
            end_cursor};
        Connection::new(store_edges, page_info)
    }

    field products_cost(&executor) -> FieldResult<f64> as "Products cost" {
        let context = executor.context();

        calculate_cart_price(context, &self.inner)
    }

    field products_cost_without_discounts(&executor) -> f64 as "Products without cost" {
        let context = executor.context();

        calculate_cart_price_without_discounts(&self.inner)
    }

    field coupons_discounts(&executor) -> FieldResult<f64> as "Coupons discounts" {
        let context = executor.context();

        calculate_cart_coupons_discount(context, &self.inner)
    }

    field delivery_cost(&executor) -> FieldResult<f64> as "Delivery cost" {
        let context = executor.context();

        calculate_cart_delivery_cost(context, &self.inner)
    }

    field total_cost(&executor) -> FieldResult<f64> as "Total cost" {
        let context = executor.context();

        Ok(calculate_cart_price(context, &self.inner)? + calculate_cart_delivery_cost(context, &self.inner)?)
    }

    field total_cost_without_discounts(&executor) -> FieldResult<f64> as "Total without cost" {
        let context = executor.context();

        Ok(calculate_cart_price_without_discounts(&self.inner) + calculate_cart_delivery_cost(context, &self.inner)?)
    }

    field total_count() -> i32 as "Total products count" {
        self.inner.iter().fold(0, |acc, store| {
            let store_products_cost = store.products.iter().fold(0, |acc, product| {
                if product.selected {
                    acc + product.quantity.0
                } else {
                    acc
                }
            });
            acc + store_products_cost
        })
    }

});

graphql_object!(CartProductStore: Context as "CartProductStore" |&self| {
    description: "Cart product store's info."

    field product_id() -> GraphqlID as "Base64 Unique product id"{
        ID::new(Service::Stores, Model::CartProduct, self.product_id.0).to_string().into()
    }

    field store_id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::CartStore, self.store_id.0).to_string().into()
    }

});

pub fn calculate_cart_price(context: &Context, stores: &[CartStore]) -> FieldResult<f64> {
    let cost = stores.iter().try_fold(0.0, |acc, store| {
        let store_products_cost = calculate_products_price(context, &store.products)?;

        Ok(acc + store_products_cost)
    });

    cost
}

pub fn calculate_cart_price_without_discounts(stores: &[CartStore]) -> f64 {
    let cost = stores.iter().fold(0.0, |acc, store| {
        let store_products_cost = calculate_products_price_without_discounts(&store.products);

        acc + store_products_cost
    });

    cost
}

pub fn calculate_cart_coupons_discount(context: &Context, stores: &[CartStore]) -> FieldResult<f64> {
    let cost = stores.iter().try_fold(0.0, |acc, store| {
        let store_coupons_discount = calculate_coupons_discount(context, &store.products)?;

        Ok(acc + store_coupons_discount)
    });

    cost
}

pub fn calculate_cart_delivery_cost(context: &Context, stores: &[CartStore]) -> FieldResult<f64> {
    let cost = stores.iter().try_fold(0.0, |acc, store| {
        let store_products_cost = calculate_products_delivery_cost(context, &store.products)?;

        Ok(acc + store_products_cost)
    });

    cost
}

pub fn run_set_delivery_method_in_cart(context: &Context, input: SetDeliveryMethodInCartInput) -> FieldResult<Cart> {
    let customer = if let Some(ref user) = context.user {
        user.user_id.into()
    } else if let Some(session_id) = context.session_id {
        session_id.into()
    } else {
        return Err(FieldError::new(
            "Could not set delivery method in cart for unauthorized user.",
            graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
        ));
    };

    let shipping_id = ShippingId(input.shipping_id);
    let product_id = ProductId(input.product_id);

    let _product: Product = product::try_get_product(context, ProductId(input.product_id))?.ok_or_else(|| {
        FieldError::new(
            "Could not set delivery method in cart.",
            graphql_value!({ "code": 100, "details": { "Product not found" }}),
        )
    })?;

    let _select_package: AvailablePackageForUser = available_packages::get_available_package_for_user_by_id(context, shipping_id)?;

    let rpc_client = context.get_rest_api_client(Service::Orders);
    let delivery_method_id = DeliveryMethodId::ShippingPackage { id: shipping_id };

    let products = rpc_client
        .set_delivery_method(customer, product_id, delivery_method_id)
        .sync()
        .map_err(into_graphql)?
        .into_iter()
        .collect::<Vec<_>>();

    convert_products_to_cart(context, &products)
}

pub fn convert_products_to_cart(context: &Context, products: &[CartItem]) -> FieldResult<Cart> {
    let url = format!("{}/{}/cart", context.config.service_url(Service::Stores), Model::Store.to_url());
    let body = serde_json::to_string(&products)?;

    context
        .request::<Vec<Store>>(Method::Post, url, Some(body))
        .map(|stores| convert_to_cart(stores, &products))
        .wait()
}
