//! File containing PageInfo object of graphql schema
use std::collections::{HashMap, HashSet};

use futures::Future;
use hyper::Method;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use stq_api::orders::DeliveryInfo;
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Translation;
use stq_types::{CartItem, DeliveryMethodId, ProductId};

use super::*;
use graphql::context::Context;
use graphql::models::*;
use graphql::schema::available_packages::*;
use graphql::schema::coupon::*;
use graphql::schema::product::*;

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

    field delivery_cost(&executor) -> FieldResult<f64> as "Delivery cost" {
        let context = executor.context();

        calculate_delivery_cost(context, &self)
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

    field deprecated "use select_package" company_package(&executor) -> FieldResult<Option<CompaniesPackages>> as "Company package" {
        let context = executor.context();
        match self.company_package_id {
            Some(company_package_id) => {
                let url = format!("{}/{}/{}",
                    context.config.service_url(Service::Delivery),
                    Model::CompanyPackage.to_url(),
                    company_package_id,
                );

                context.request::<Option<CompaniesPackages>>(Method::Get, url, None).wait()
            },
            None => Ok(None),
        }
    }

    field select_package(&executor) -> FieldResult<Option<AvailablePackageForUser>> as "Select package" {
        let context = executor.context();

        match self.delivery_method_id {
            Some(delivery_method_id) =>  Ok(Some(get_select_package(context, delivery_method_id, self.id)?)),
            _ => Ok(None),
        }
    }

    field deprecated "use companyPackage" delivery_operator() -> &str as "Delivery Operator" {
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

    field attributes(&executor) -> FieldResult<Option<Vec<ProdAttrValue>>> as "Variants" {
        let context = executor.context();
        let url = format!("{}/{}/{}/attributes",
            context.config.service_url(Service::Stores),
            Model::Product.to_url(),
            self.id);

        context.request::<Vec<ProdAttrValue>>(Method::Get, url, None)
            .wait()
            .or_else(|_| Ok(vec![]))
            .map(Some)
    }
});

pub fn calculate_product_price(context: &Context, cart_product: &CartProduct) -> FieldResult<f64> {
    if cart_product.quantity.0 <= 0 {
        return Ok(0f64);
    }

    if let Some(discount) = cart_product.discount.filter(|discount| *discount > ZERO_DISCOUNT) {
        let calc_price = (cart_product.price.0 * (f64::from(cart_product.quantity.0))) * (1.0f64 - discount);

        return Ok(calc_price);
    } else {
        if cart_product.coupon_id.is_some() {
            // set discount only 1 product
            let product_price_with_coupon_discount = cart_product.price.0 - calculate_coupon_discount(context, cart_product)?;
            let calc_price = product_price_with_coupon_discount + (cart_product.price.0 * (f64::from(cart_product.quantity.0 - 1)));

            return Ok(calc_price);
        }
    }

    Ok(cart_product.price.0 * f64::from(cart_product.quantity.0))
}

pub fn calculate_product_price_without_discounts(product: &CartProduct) -> f64 {
    if product.quantity.0 <= 0 {
        return 0f64;
    }

    product.price.0 * f64::from(product.quantity.0)
}

pub fn calculate_coupon_discount(context: &Context, cart_product: &CartProduct) -> FieldResult<f64> {
    if let Some(coupon_id) = cart_product.coupon_id {
        if let Some(coupon) = try_get_coupon(context, coupon_id)? {
            // set discount only 1 product
            let discount = (cart_product.price.0 / 100f64) * f64::from(coupon.percent);

            return Ok(discount);
        }
    }

    Ok(0.0f64)
}

pub fn calculate_delivery_cost(context: &Context, product: &CartProduct) -> FieldResult<f64> {
    if let Some(delivery_method_id) = product.delivery_method_id {
        let package = get_select_package(context, delivery_method_id, product.id)?;

        if let Some(price) = package.price {
            return Ok(price.0 * f64::from(product.quantity.0));
        }
    }

    Ok(0.0f64)
}

pub fn get_selected_packages<'a>(
    context: &Context,
    cart_items: &'a HashSet<CartItem>,
) -> FieldResult<HashMap<&'a CartItem, AvailablePackageForUser>> {
    let mut packages = vec![];

    for cart_item in cart_items.iter() {
        if let Some(delivery_method_id) = cart_item.delivery_method_id {
            let package = get_select_package(context, delivery_method_id, cart_item.product_id)?;
            packages.push((cart_item, package));
        }
    }

    Ok(packages.into_iter().collect())
}

pub fn get_delivery_info(packages: HashMap<ProductId, AvailablePackageForUser>) -> FieldResult<HashMap<ProductId, DeliveryInfo>> {
    let delivery_info = packages
        .into_iter()
        .map(|(product_id, package)| {
            let element = available_packages::get_delivery_info(package);

            (product_id, element)
        }).collect::<HashMap<ProductId, DeliveryInfo>>();

    Ok(delivery_info)
}

fn get_select_package(context: &Context, delivery_method: DeliveryMethodId, product_id: ProductId) -> FieldResult<AvailablePackageForUser> {
    match delivery_method {
        DeliveryMethodId::Package { id: company_package_id } => {
            let product = get_product(context, product_id)?;
            get_available_package_for_user(context, product.base_product_id, company_package_id)
        }
        DeliveryMethodId::ShippingPackage { id: shipping_id } => get_available_package_for_user_by_id(context, shipping_id),
        _ => Err(FieldError::new(
            "Could not get select package.",
            graphql_value!({ "code": 100, "details": { "Delivery method not support." }}),
        )),
    }
}

pub fn validate_select_package(cart_product: &CartItem, package: &AvailablePackageForUser) -> FieldResult<()> {
    if cart_product.store_id != package.store_id {
        return Err(FieldError::new(
            "Select package not valid.",
            graphql_value!({ "code": 100, "details": { "The selected package is not found in the store." }}),
        ));
    }

    Ok(())
}
