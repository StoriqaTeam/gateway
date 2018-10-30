//! File containing PageInfo object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Translation;

use super::*;
use graphql::context::Context;
use graphql::models::*;
use graphql::schema::coupon::*;

graphql_object!(CartProduct: Context as "CartProduct" |&self| {
    description: "Cart Product info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Orders, Model::CartProduct, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id.0
    }

    field name() -> &[Translation] as "Full Name" {
        &self.name
    }

    field quantity() -> &i32 as "Quantity" {
        &self.quantity.0
    }

    field price() -> &f64 as "Price" {
        &self.price.0
    }

    field subtotal(&executor) -> FieldResult<f64> as "Subtotal with discounts" {
        let context = executor.context();
        calculate_product_price(context, &self)
    }

    field subtotal_without_discounts() -> f64 as "Subtotal without discounts" {
        self.price.0 * f64::from(self.quantity.0)
    }

    field delivery_cost() -> f64 as "Delivery cost" {
        0.0
    }

    field photo_main() -> &Option<String> as "Photo main" {
        &self.photo_main
    }

    field comment() -> &str as "Comment" {
        &self.comment
    }

    field selected() -> &bool as "Selected" {
        &self.selected
    }

    field delivery_operator() -> &str as "Delivery Operator" {
        "Operator"
    }

    field delivery_period() -> &str as "Delivery Period" {
        "14 days"
    }

    field delivery_return_type() -> &str as "Delivery return type" {
        "funds return"
    }

    field delivery_return_paid_by() -> &str as "Delivery return paid by" {
        "Seller"
    }

    field pre_order() -> &bool as "Pre order" {
        &self.pre_order
    }

    field pre_order_days() -> &i32 as "Pre order days" {
        &self.pre_order_days
    }

    field coupon(&executor) -> FieldResult<Option<Coupon>> as "Coupon added user" {
        let context = executor.context();
        if let Some(coupon_id) = self.coupon_id {
            try_get_coupon(context, coupon_id)
        } else {
            Ok(None)
        }
    }

    field coupon_discount(&executor) -> FieldResult<f64> as "Coupon discount" {
        let context = executor.context();

        calculate_coupon_discount(context, &self)
    }

    field base_product(&executor,
        visibility: Option<Visibility> as "Specifies allowed visibility of the base_product"
    ) -> FieldResult<Option<BaseProduct>> as "Fetches base product by product." {
        let context = executor.context();
        let visibility = visibility.unwrap_or_default();

        let url = format!(
            "{}/{}/{}?visibility={}",
            &context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            self.base_product_id.to_string(),
            visibility,
        );

        context.request::<Option<BaseProduct>>(Method::Get, url, None)
            .wait()
    }

    field base_product_id() -> &i32 as "BaseProductId" {
        &self.base_product_id.0
    }

    field attributes(&executor) -> FieldResult<Option<Vec<AttrValue>>> as "Variants" {
        let context = executor.context();
        let url = format!("{}/{}/{}/attributes",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.id);

        context.request::<Vec<AttrValue>>(Method::Get, url, None)
            .wait()
            .or_else(|_| Ok(vec![]))
            .map(Some)
    }
});

pub fn calculate_product_price(context: &Context, product: &CartProduct) -> FieldResult<f64> {
    if product.quantity.0 <= 0 {
        return Ok(0f64);
    }

    if let Some(coupon_id) = product.coupon_id {
        if let Some(coupon) = try_get_coupon(context, coupon_id)? {
            // set discount only 1 product
            let set_discount = (product.price.0 * 1f64) - ((product.price.0 / 100f64) * f64::from(coupon.percent));
            let calc_price = set_discount + (product.price.0 * (f64::from(product.quantity.0) - 1f64));

            return Ok(calc_price);
        }
    }

    Ok(product.price.0 * f64::from(product.quantity.0))
}

pub fn calculate_product_price_without_discounts(product: &CartProduct) -> f64 {
    if product.quantity.0 <= 0 {
        return 0f64;
    }

    product.price.0 * f64::from(product.quantity.0)
}

pub fn calculate_coupon_discount(context: &Context, product: &CartProduct) -> FieldResult<f64> {
    let price_with_discounts = calculate_product_price(context, product)?;

    Ok(calculate_product_price_without_discounts(product) - price_with_discounts)
}
