//! File containing buy now values object of graphql schema

use juniper::{FieldError, FieldResult};

use futures::Future;
use hyper::Method;

use stq_static_resources::Currency;
use stq_types::*;

use graphql::context::Context;
use graphql::models::*;
use graphql::schema::available_packages::*;
use graphql::schema::coupon;
use graphql::schema::order;
use graphql::schema::product as product_module;
use graphql::schema::store;
use graphql::schema::user::get_user_by_id;

graphql_object!(BuyNowCheckout: Context as "BuyNowCheckout" |&self| {
    description: "buy now values info."

    field product() -> &Product as "Product" {
        &self.product
    }

    field coupon() -> &Option<Coupon> as "Coupon added user" {
        &self.coupon
    }

    field coupons_discounts() -> f64 as "Coupons discounts" {
        calculate_coupon_discount(&self)
    }

    field total_cost() -> f64 as "Total cost" {
        calculate_cost(&self) + calculate_delivery_cost(&self.package, self.quantity)
    }

    field total_cost_without_discounts() -> f64 as "Total without cost" {
        calculate_cost_without_discounts(&self) + calculate_delivery_cost(&self.package, self.quantity)
    }

    field total_count() -> &i32 as "Total products count" {
        &self.quantity.0
    }

    field price() -> &f64 as "Price" {
        &self.product.price.0
    }

    field subtotal() -> f64 as "Subtotal with discounts" {
        calculate_cost(&self)
    }

    field subtotal_without_discounts() -> f64 as "Subtotal without discounts" {
        calculate_cost_without_discounts(&self)
    }

    field delivery_cost() -> f64 as "Delivery cost" {
        calculate_delivery_cost(&self.package, self.quantity)
    }

    field package() -> &Option<AvailablePackageForUser> as "Select delivery package" {
        &self.package
    }
});

fn calculate_cost(buy_now: &BuyNowCheckout) -> f64 {
    if buy_now.quantity.0 <= 0 {
        return 0f64;
    }

    if let Some(discount) = buy_now.product.discount.filter(|discount| *discount > ZERO_DISCOUNT) {
        let calc_cost = (buy_now.product.price.0 * (f64::from(buy_now.quantity.0))) * (1.0f64 - discount);

        return calc_cost;
    } else {
        if buy_now.coupon.is_some() {
            // set discount only 1 product
            let product_cost_with_coupon_discount = buy_now.product.price.0 - calculate_coupon_discount(buy_now);
            let calc_cost = product_cost_with_coupon_discount + (buy_now.product.price.0 * (f64::from(buy_now.quantity.0 - 1)));

            return calc_cost;
        }
    }

    buy_now.product.price.0 * f64::from(buy_now.quantity.0)
}

fn calculate_cost_without_discounts(buy_now: &BuyNowCheckout) -> f64 {
    if buy_now.quantity.0 <= 0 {
        return 0f64;
    }

    buy_now.product.price.0 * f64::from(buy_now.quantity.0)
}

fn calculate_coupon_discount(buy_now: &BuyNowCheckout) -> f64 {
    if let Some(coupon) = buy_now.coupon.as_ref() {
        // set discount only 1 product
        let discount = (buy_now.product.price.0 / 100f64) * f64::from(coupon.percent);

        return discount;
    }

    0.0f64
}

fn calculate_delivery_cost(package: &Option<AvailablePackageForUser>, quantity: Quantity) -> f64 {
    if let Some(package) = package {
        return calculate_delivery(package.price, quantity);
    }

    0.0f64
}

fn calculate_delivery(price: ProductPrice, quantity: Quantity) -> f64 {
    if quantity.0 <= 0 {
        return 0f64;
    }

    price.0 * f64::from(quantity.0)
}

pub fn run_buy_now_mutation(context: &Context, input: BuyNowInputV2) -> FieldResult<CreateOrdersOutput> {
    let input = input.fill_uuid();
    let user = context.user.clone().ok_or_else(|| {
        FieldError::new(
            "Could not run for unauthorized user.",
            graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
        )
    })?;

    let product_price = product_module::get_seller_price(context, ProductId(input.product_id))?;
    let store_id = store::get_store_id_by_product(context, ProductId(input.product_id))?;
    let product = product_module::get_product(context, ProductId(input.product_id))?;

    let (shipping_details, package) = get_available_package_for_user_with_price(
        context,
        product.base_product_id,
        ShippingId(input.shipping_id),
        input.user_country_code.as_str(),
        "Buy Now failed.",
    )?;

    let coupon = match input.coupon_code {
        Some(code) => {
            let coupon = validate_coupon(context, CouponCode(code), ProductId(input.product_id), store_id)?;
            Some(coupon)
        }
        None => None,
    };

    let customer = get_user_by_id(context, user.user_id)?;
    let delivery_info = get_delivery_info(package);
    let product_info = ProductInfo::from(product.clone());

    let buy_now = BuyNow {
        product_id: input.product_id.into(),
        store_id: shipping_details.store_id,
        customer_id: user.user_id,
        address: input.address_full.into(),
        receiver_name: input.receiver_name,
        receiver_phone: input.receiver_phone,
        receiver_email: customer.email,
        price: product_price,
        quantity: input.quantity.into(),
        currency: input.currency,
        pre_order: product.pre_order,
        pre_order_days: product.pre_order_days,
        coupon,
        delivery_info: Some(delivery_info),
        product_info,
        uuid: input.uuid,
    };

    if buy_now.currency == Currency::EUR || buy_now.currency == Currency::EUR || buy_now.currency == Currency::USD {
        order::validate_products_fiat([buy_now.price.clone()].iter())?;
    }

    let url = format!("{}/buy_now", context.config.saga_microservice.url.clone());
    let body: String = serde_json::to_string(&buy_now)?.to_string();
    context
        .request::<Invoice>(Method::Post, url, Some(body))
        .wait()
        .map(CreateOrdersOutput)
}

/// DEPRECATED
pub fn run_buy_now_mutation_v1(context: &Context, input: BuyNowInput) -> FieldResult<CreateOrdersOutput> {
    let input = input.fill_uuid();
    let user = context.user.clone().ok_or_else(|| {
        FieldError::new(
            "Could not run for unauthorized user.",
            graphql_value!({ "code": 100, "details": { "No user id in request header." }}),
        )
    })?;

    let product_price = product_module::get_seller_price(context, ProductId(input.product_id))?;
    let store_id = store::get_store_id_by_product(context, ProductId(input.product_id))?;
    let product = product_module::get_product(context, ProductId(input.product_id))?;

    let coupon = match input.coupon_code {
        Some(code) => {
            let coupon = validate_coupon(context, CouponCode(code), ProductId(input.product_id), store_id)?;
            Some(coupon)
        }
        None => None,
    };

    let customer = get_user_by_id(context, user.user_id)?;
    let package = get_available_package_for_user_by_id_v1(context, ShippingId(input.shipping_id))?;

    if store_id != package.store_id {
        return Err(FieldError::new(
            "Select package not valid.",
            graphql_value!({ "code": 100, "details": { "The selected package is not found in the store." }}),
        ));
    }

    let delivery_info = get_delivery_info(package);
    let product_info = ProductInfo::from(product.clone());

    let buy_now = BuyNow {
        product_id: input.product_id.into(),
        store_id,
        customer_id: user.user_id,
        address: input.address_full,
        receiver_name: input.receiver_name,
        receiver_phone: input.receiver_phone,
        receiver_email: customer.email,
        price: product_price,
        quantity: input.quantity.into(),
        currency: input.currency,
        pre_order: product.pre_order,
        pre_order_days: product.pre_order_days,
        coupon,
        delivery_info: Some(delivery_info),
        product_info,
        uuid: input.uuid,
    };

    if buy_now.currency == Currency::EUR || buy_now.currency == Currency::EUR || buy_now.currency == Currency::USD {
        order::validate_products_fiat([buy_now.price.clone()].iter())?;
    }

    let url = format!("{}/buy_now", context.config.saga_microservice.url.clone());
    let body: String = serde_json::to_string(&buy_now)?.to_string();
    context
        .request::<Invoice>(Method::Post, url, Some(body))
        .wait()
        .map(CreateOrdersOutput)
}

pub fn calculate_buy_now_v1(
    context: &Context,
    product_id: i32,
    quantity: i32,
    coupon_code: Option<String>,
    shipping_id: Option<i32>,
) -> FieldResult<BuyNowCheckout> {
    let store_id = store::get_store_id_by_product(context, ProductId(product_id))?;
    let product = product_module::get_product(context, ProductId(product_id))?;

    let coupon = match coupon_code {
        Some(code) => {
            let coupon = validate_coupon(context, CouponCode(code), ProductId(product_id), store_id)?;
            Some(coupon)
        }
        None => None,
    };

    let package = match shipping_id {
        Some(shipping_id) => {
            let result = get_available_package_for_user_by_id_v1(context, ShippingId(shipping_id))?;

            Some(result)
        }
        _ => None,
    };

    Ok(BuyNowCheckout {
        user_country_code: None,
        product,
        quantity: quantity.into(),
        coupon,
        package,
    })
}

pub fn calculate_buy_now(
    context: &Context,
    product_id: i32,
    quantity: i32,
    user_country_code: &str,
    coupon_code: Option<String>,
    shipping_id: Option<i32>,
) -> FieldResult<BuyNowCheckout> {
    let store_id = store::get_store_id_by_product(context, ProductId(product_id))?;
    let product = product_module::get_product(context, ProductId(product_id))?;

    let package = match shipping_id {
        None => None,
        Some(shipping_id) => {
            let (_shipping_details, package) = get_available_package_for_user_with_price(
                context,
                product.base_product_id,
                ShippingId(shipping_id),
                user_country_code,
                "Could not calculate buy now.",
            )?;
            Some(package)
        }
    };

    let coupon = match coupon_code {
        Some(code) => {
            let coupon = validate_coupon(context, CouponCode(code), ProductId(product_id), store_id)?;
            Some(coupon)
        }
        None => None,
    };

    Ok(BuyNowCheckout {
        user_country_code: Some(user_country_code.to_string()),
        product,
        quantity: quantity.into(),
        coupon,
        package,
    })
}

fn validate_coupon(context: &Context, coupon_code: CouponCode, product_id: ProductId, store_id: StoreId) -> FieldResult<Coupon> {
    coupon::validate_coupon_by_code(context, coupon_code.clone(), store_id)?;
    let coupon = coupon::get_coupon_by_code(context, coupon_code, store_id)?;

    let all_support_products = coupon::get_products(context, coupon.id)?
        .into_iter()
        .filter(|p| match p.discount {
            Some(discount) => discount < ZERO_DISCOUNT,
            None => true,
        })
        .filter(|p| p.id == product_id)
        .collect::<Vec<Product>>();

    if all_support_products.is_empty() {
        return Err(FieldError::new(
            "Coupon not set",
            graphql_value!({ "code": 400, "details": { "no products found for coupon usage" }}),
        ));
    }

    Ok(coupon)
}
