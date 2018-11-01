//! File containing buy now values object of graphql schema
use juniper::FieldResult;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(BuyNowCheckout: Context as "BuyNowCheckout" |&self| {
    description: "buy now values info."

    field product() -> &Product as "Product" {
        &self.product
    }

    field coupon() -> &Option<Coupon> as "Coupon added user" {
        &self.coupon
    }

    field coupons_discounts() -> FieldResult<f64> as "Coupons discounts" {
        calculate_coupon_discount(&self)
    }

    field total_cost() -> FieldResult<f64> as "Total cost" {
        calculate_cost(&self)
    }

    field total_cost_without_discounts() -> f64 as "Total without cost" {
        calculate_cost_without_discounts(&self)
    }

    field total_count() -> &i32 as "Total products count" {
        &self.quantity.0
    }

    field delivery_cost() -> f64 as "Delivery cost" {
        0.0
    }
});

fn calculate_cost(buy_now: &BuyNowCheckout) -> FieldResult<f64> {
    if buy_now.quantity.0 <= 0 {
        return Ok(0f64);
    }

    if let Some(coupon) = buy_now.coupon.as_ref() {
        // set discount only 1 product
        let set_discount = (buy_now.product.price.0 * 1f64) - ((buy_now.product.price.0 / 100f64) * f64::from(coupon.percent));
        let calc_cost = set_discount + (buy_now.product.price.0 * (f64::from(buy_now.quantity.0) - 1f64));

        return Ok(calc_cost);
    }

    Ok(buy_now.product.price.0 * f64::from(buy_now.quantity.0))
}

fn calculate_cost_without_discounts(buy_now: &BuyNowCheckout) -> f64 {
    if buy_now.quantity.0 <= 0 {
        return 0f64;
    }

    buy_now.product.price.0 * f64::from(buy_now.quantity.0)
}

fn calculate_coupon_discount(buy_now: &BuyNowCheckout) -> FieldResult<f64> {
    let cost_with_discounts = calculate_cost(buy_now)?;

    Ok(calculate_cost_without_discounts(buy_now) - cost_with_discounts)
}
