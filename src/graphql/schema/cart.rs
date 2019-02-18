//! File containing Cart object of graphql schema

use std::cmp;
use std::str::FromStr;

use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use futures::Future;
use hyper::Method;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_types::{CartCustomer, CartItem, DeliveryMethodId, ProductId, Quantity, ShippingId, UserId};

use stq_api::orders::CartClient;
use stq_api::types::ApiFutureExt;

use stq_static_resources::CurrencyType;

use super::*;
use graphql::context::Context;
use graphql::models::*;
use graphql::schema::cart_store::{
    calculate_coupons_discount, calculate_products_delivery_cost, calculate_products_price, calculate_products_price_without_discounts,
};

use errors::into_graphql;
use graphql::schema::available_packages;
use graphql::schema::base_product as base_product_module;
use graphql::schema::product as product_module;

graphql_object!(Cart: Context as "Cart" |&self| {
    description: "Users cart"

    interfaces: [&Node]

    field id(&executor) -> GraphqlID as "Base64 Unique id"{
        let context = executor.context();

        let currency_type_str = match self.currency_type {
            Some(c) => c.to_string(),
            None => "".to_string(),
        };

        let id_str = if let Some(ref user) = context.user {
            ID::new(Service::Orders, Model::Cart, user.user_id.0).to_string()
        } else if let Some(session_id) = context.session_id {
            session_id.0.to_string()
        }  else {
            ID::new(Service::Orders, Model::Cart, UserId::default().0).to_string()
        };

        format!("{}|{}", id_str, currency_type_str).into()
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
    field fiat(&executor) -> FieldResult<Cart> as "Fiat cart" {
        let context = executor.context();
        get_cart(context, Some(CurrencyType::Fiat))

    }
    field crypto(&executor) -> FieldResult<Cart> as "Crypto cart" {
        let context = executor.context();
        get_cart(context, Some(CurrencyType::Crypto))
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

pub fn run_set_delivery_method_in_cart(context: &Context, input: SetDeliveryMethodInCartInputV2) -> FieldResult<Cart> {
    let customer: CartCustomer = get_customer(context).ok_or_else(|| {
        FieldError::new(
            "Could not set delivery method in cart for unauthorized user.",
            graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
        )
    })?;

    let shipping_id = ShippingId(input.shipping_id);
    let product_id = ProductId(input.product_id);

    let product = product_module::try_get_product(context, product_id)?.ok_or_else(|| {
        FieldError::new(
            "Could not set delivery method in cart.",
            graphql_value!({ "code": 100, "details": { "Product not found" }}),
        )
    })?;

    available_packages::get_available_package_for_user_with_price(
        context,
        product.base_product_id,
        shipping_id,
        input.user_country_code.as_str(),
        "Could not set delivery method in cart.",
    )?;

    let rpc_client = context.get_rest_api_client(Service::Orders);
    let delivery_method_id = DeliveryMethodId::ShippingPackage { id: shipping_id };

    let products = rpc_client
        .set_delivery_method(customer, product_id, delivery_method_id)
        .sync()
        .map_err(into_graphql)?
        .into_iter()
        .collect::<Vec<_>>();

    convert_products_to_cart(context, &products, Some(input.user_country_code))
}

pub fn run_set_delivery_method_in_cart_v1(context: &Context, input: SetDeliveryMethodInCartInput) -> FieldResult<Cart> {
    let customer: CartCustomer = get_customer(context).ok_or_else(|| {
        FieldError::new(
            "Could not set delivery method in cart for unauthorized user.",
            graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
        )
    })?;

    let shipping_id = ShippingId(input.shipping_id);
    let product_id = ProductId(input.product_id);

    let _product: Product = product_module::try_get_product(context, product_id)?.ok_or_else(|| {
        FieldError::new(
            "Could not set delivery method in cart.",
            graphql_value!({ "code": 100, "details": { "Product not found" }}),
        )
    })?;

    let _select_package: AvailablePackageForUser = available_packages::get_available_package_for_user_by_id_v1(context, shipping_id)?;

    let rpc_client = context.get_rest_api_client(Service::Orders);
    let delivery_method_id = DeliveryMethodId::ShippingPackage { id: shipping_id };

    let products = rpc_client
        .set_delivery_method(customer, product_id, delivery_method_id)
        .sync()
        .map_err(into_graphql)?
        .into_iter()
        .collect::<Vec<_>>();

    convert_products_to_cart(context, &products, None)
}

pub fn run_remove_delivery_method_from_cart_v1(context: &Context, input: RemoveDeliveryMethodFromCartInput) -> FieldResult<Cart> {
    let customer: CartCustomer = get_customer(context).ok_or_else(|| {
        FieldError::new(
            "Could not remove delivery method from cart for unauthorized user.",
            graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
        )
    })?;

    let product_id = ProductId(input.product_id);

    let _product: Product = product_module::try_get_product(context, product_id)?.ok_or_else(|| {
        FieldError::new(
            "Could not remove delivery method from cart.",
            graphql_value!({ "code": 100, "details": { "Product not found" }}),
        )
    })?;

    let rpc_client = context.get_rest_api_client(Service::Orders);
    let products = rpc_client
        .delete_delivery_method_by_product(customer, ProductId(input.product_id))
        .sync()
        .map_err(into_graphql)?
        .into_iter()
        .collect::<Vec<_>>();

    convert_products_to_cart(context, &products, None)
}

pub fn run_remove_delivery_method_from_cart(context: &Context, input: RemoveDeliveryMethodFromCartInputV2) -> FieldResult<Cart> {
    let customer: CartCustomer = get_customer(context).ok_or_else(|| {
        FieldError::new(
            "Could not remove delivery method from cart for unauthorized user.",
            graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
        )
    })?;

    let product_id = ProductId(input.product_id);

    let _product: Product = product_module::try_get_product(context, product_id)?.ok_or_else(|| {
        FieldError::new(
            "Could not remove delivery method from cart.",
            graphql_value!({ "code": 100, "details": { "Product not found" }}),
        )
    })?;

    let rpc_client = context.get_rest_api_client(Service::Orders);
    let products = rpc_client
        .delete_delivery_method_by_product(customer, ProductId(input.product_id))
        .sync()
        .map_err(into_graphql)?
        .into_iter()
        .collect::<Vec<_>>();

    convert_products_to_cart(context, &products, Some(input.user_country_code))
}

pub fn run_increment_in_cart_v1(context: &Context, input: IncrementInCartInput) -> FieldResult<Option<Cart>> {
    let customer: CartCustomer = get_customer(context).ok_or_else(|| {
        FieldError::new(
            "Could not increment cart for unauthorized user.",
            graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
        )
    })?;

    let product_id = ProductId(input.product_id);

    let base_product = base_product_module::get_base_product_by_product(context, product_id)?;

    let product = base_product.variants.and_then(|v| v.get(0).cloned()).ok_or_else(|| {
        FieldError::new(
            "Could not find product in base product variants.",
            graphql_value!({ "code": 100, "details": { "Product does not exist in variants." }}),
        )
    })?;

    let rpc_client = context.get_rest_api_client(Service::Orders);
    let init_quantity = rpc_client
        .get_cart(customer, Some(base_product.currency.currency_type()))
        .sync()
        .map_err(into_graphql)?
        .into_iter()
        .find(|product| product.product_id == product_id)
        .map(|product| product.quantity.0)
        .unwrap_or(0i32);

    let rpc_client = context.get_rest_api_client(Service::Orders);

    let mut products: Vec<_> = rpc_client
        .increment_item(
            customer,
            input.product_id.into(),
            base_product.store_id,
            product.pre_order,
            product.pre_order_days,
            base_product.currency.currency_type(),
            None
        )
        .sync()
        .map_err(into_graphql)?
        .into_iter()
        .collect();
    // drop previous rpc_client
    let rpc_client = context.get_rest_api_client(Service::Orders);
    if let Some(value) = input.value {
        let quantity = Quantity(init_quantity + value);
        products = rpc_client
            .set_quantity(customer, input.product_id.into(), quantity)
            .sync()
            .map_err(into_graphql)?
            .into_iter()
            .collect();
    }

    convert_products_to_cart(context, &products, None).map(Some)
}

pub fn run_increment_in_cart(context: &Context, input: IncrementInCartInputV2) -> FieldResult<Option<Cart>> {
    let customer: CartCustomer = get_customer(context).ok_or_else(|| {
        FieldError::new(
            "Could not increment cart for unauthorized user.",
            graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
        )
    })?;

    let product_id = ProductId(input.product_id);

    let base_product = base_product_module::get_base_product_by_product(context, product_id)?;

    let product = base_product.variants.and_then(|v| v.get(0).cloned()).ok_or_else(|| {
        FieldError::new(
            "Could not find product in base product variants.",
            graphql_value!({ "code": 100, "details": { "Product does not exist in variants." }}),
        )
    })?;

    let rpc_client = context.get_rest_api_client(Service::Orders);
    let init_quantity = rpc_client
        .get_cart(customer, Some(base_product.currency.currency_type()))
        .sync()
        .map_err(into_graphql)?
        .into_iter()
        .find(|product| product.product_id == product_id)
        .map(|product| product.quantity.0)
        .unwrap_or(0i32);

    // drop previous rpc_client
    let rpc_client = context.get_rest_api_client(Service::Orders);

    let mut products: Vec<_> = rpc_client
        .increment_item(
            customer,
            input.product_id.into(),
            base_product.store_id,
            product.pre_order,
            product.pre_order_days,
            base_product.currency.currency_type(),
            Some(input.user_country_code.clone().into())
        )
        .sync()
        .map_err(into_graphql)?
        .into_iter()
        .collect();

    // drop previous rpc_client
    let rpc_client = context.get_rest_api_client(Service::Orders);
    if let Some(value) = input.value {
        let quantity = Quantity(init_quantity + value);
        products = rpc_client
            .set_quantity(customer, input.product_id.into(), quantity)
            .sync()
            .map_err(into_graphql)?
            .into_iter()
            .collect();
    }

    convert_products_to_cart(context, &products, Some(input.user_country_code)).map(Some)
}

pub fn run_add_in_cart_v1(context: &Context, input: AddInCartInput) -> FieldResult<Option<Cart>> {
    let input_increment = IncrementInCartInput {
        client_mutation_id: input.client_mutation_id.clone(),
        product_id: input.product_id,
        value: input.value,
    };

    run_increment_in_cart_v1(context, input_increment).and_then(|inc_cart| {
        if let Some(shipping_id) = input.shipping_id {
            let input_delivery_method = SetDeliveryMethodInCartInput {
                client_mutation_id: input.client_mutation_id,
                product_id: input.product_id,
                company_package_id: None,
                shipping_id,
            };

            run_set_delivery_method_in_cart_v1(context, input_delivery_method).map(Some)
        } else {
            Ok(inc_cart)
        }
    })
}

pub fn run_add_in_cart(context: &Context, input: AddInCartInputV2) -> FieldResult<Option<Cart>> {
    let input_increment = IncrementInCartInputV2 {
        client_mutation_id: input.client_mutation_id.clone(),
        product_id: input.product_id,
        value: input.value,
        user_country_code: input.user_country_code.clone(),
    };

    run_increment_in_cart(context, input_increment).and_then(|inc_cart| {
        if let Some(shipping_id) = input.shipping_id {
            let input_delivery_method = SetDeliveryMethodInCartInputV2 {
                client_mutation_id: input.client_mutation_id,
                product_id: input.product_id,
                shipping_id,
                user_country_code: input.user_country_code,
            };

            run_set_delivery_method_in_cart(context, input_delivery_method).map(Some)
        } else {
            Ok(inc_cart)
        }
    })
}

pub fn convert_products_to_cart(context: &Context, products: &[CartItem], user_country_code: Option<String>) -> FieldResult<Cart> {
    let url = format!("{}/{}/cart", context.config.service_url(Service::Stores), Model::Store.to_url());
    let body = serde_json::to_string(&products)?;

    context
        .request::<Vec<Store>>(Method::Post, url, Some(body))
        .map(|stores| convert_to_cart(stores, &products, user_country_code))
        .wait()
}

pub fn get_cart(context: &Context, currency_type: Option<CurrencyType>) -> FieldResult<Cart> {
    let rpc_client = context.get_rest_api_client(Service::Orders);
    let fut = if let Some(session_id) = context.session_id {
        if let Some(ref user) = context.user {
            rpc_client.merge(session_id.into(), user.user_id.into(), currency_type)
        } else {
            rpc_client.get_cart(session_id.into(), currency_type)
        }
    } else if let Some(ref user) = context.user {
        rpc_client.get_cart(user.user_id.into(), currency_type)
    } else {
        return Err(FieldError::new(
            "Could not get users cart.",
            graphql_value!({ "code": 100, "details": { "No user id or session id in request header." }}),
        ));
    };

    let products: Vec<_> = fut.sync().map_err(into_graphql)?.into_iter().collect();

    let mut cart = convert_products_to_cart(context, &products, None)?;
    cart.currency_type = currency_type;
    Ok(cart)
}

pub fn get_customer(context: &Context) -> Option<CartCustomer> {
    if let Some(ref user) = context.user {
        Some(user.user_id.into())
    } else if let Some(session_id) = context.session_id {
        Some(session_id.into())
    } else {
        None
    }
}
