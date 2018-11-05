//! File containing buy now values object of graphql schema

use stq_api::orders::DeliveryInfo;
use stq_types::{ProductPrice, Quantity};

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
        if let Some(price) = package.price {
            return calculate_delivery(price, quantity);
        }
    }

    0.0f64
}

fn calculate_delivery(price: ProductPrice, quantity: Quantity) -> f64 {
    if quantity.0 <= 0 {
        return 0f64;
    }

    price.0 * f64::from(quantity.0)
}

pub fn get_delivery_info(package: AvailablePackageForUser) -> DeliveryInfo {
    let price = match package.price {
        Some(price) => price.0,
        _ => 0.0f64,
    };

    DeliveryInfo {
        company_package_id: package.id,
        shipping_id: package.shipping_id,
        name: package.name,
        logo: package.logo,
        price,
    }
}
