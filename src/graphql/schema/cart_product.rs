//! File containing PageInfo object of graphql schema
use std::collections::{HashMap, HashSet};

use futures::Future;
use hyper::Method;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::currency_type::CurrencyType;
use stq_static_resources::Currency;
use stq_static_resources::Translation;
use stq_types::{BaseProductId, CartItem, DeliveryMethodId, ExchangeRate, ProductId};

use super::*;
use graphql::context::Context;
use graphql::models::*;
use graphql::schema::available_packages::*;
use graphql::schema::coupon::*;
use graphql::schema::product as product_module;

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

    field deprecated "use selectPackage" company_package(&executor) -> FieldResult<Option<CompaniesPackages>> as "Company package" {
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


        Ok(match self.delivery_method_id {
            Some(delivery_method_id) => match self.user_country_code.clone() {
                Some(user_country_code) => Some(get_select_package(context, self.base_product_id, user_country_code, delivery_method_id)?),
                None => Some(get_select_package_v1(context, delivery_method_id)?),
            },
            _ => None,
        })
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

pub fn get_cart_product_base_product(context: &Context, product: &CartProduct) -> FieldResult<Option<BaseProduct>> {
    let url = format!(
        "{}/{}/{}?visibility={}",
        &context.config.service_url(Service::Stores),
        Model::BaseProduct.to_url(),
        product.base_product_id.to_string(),
        Visibility::Published,
    );

    context.request::<Option<BaseProduct>>(Method::Get, url, None).wait()
}

pub fn get_currency_exchange_rates(context: &Context, currency: Currency) -> FieldResult<ExchangeRates> {
    Ok(context
        .get_stores_microservice()
        .get_currency_exchange_info()?
        .data
        .get(&currency)
        .cloned()
        .unwrap_or_default())
}

pub fn get_exchange_rate(context: &Context, product: &CartProduct) -> FieldResult<ExchangeRate> {
    let user_currency = match product.currency.currency_type() {
        CurrencyType::Crypto => context.currency,
        CurrencyType::Fiat => context.fiat_currency,
    }
    .unwrap_or(product.currency);
    let currency_map = get_currency_exchange_rates(context, product.currency)?;
    Ok(currency_map.get(&user_currency).cloned().unwrap_or(ExchangeRate(1.0)))
}

pub fn calculate_delivery_cost(context: &Context, product: &CartProduct) -> FieldResult<f64> {
    calculate_delivery_cost_with_exchange_rate(context, product, get_exchange_rate(context, product)?)
}

pub fn calculate_delivery_cost_with_exchange_rate(
    context: &Context,
    product: &CartProduct,
    exchange_rate: ExchangeRate,
) -> FieldResult<f64> {
    match product.delivery_method_id {
        None => Ok(0.0f64),
        Some(delivery_method_id) => {
            let package = match product.user_country_code.clone() {
                None => get_select_package_v1(context, delivery_method_id)?,
                Some(user_country_code) => get_select_package(context, product.base_product_id, user_country_code, delivery_method_id)?,
            };
            Ok(package.price.0 / exchange_rate.0 * f64::from(product.quantity.0))
        }
    }
}

pub fn get_selected_packages<'a>(
    context: &Context,
    cart_items: &'a HashSet<CartItem>,
    user_country_code: Option<String>,
) -> FieldResult<HashMap<&'a CartItem, AvailablePackageForUser>> {
    let mut packages = vec![];

    for cart_item in cart_items.iter() {
        if let Some(delivery_method_id) = cart_item.delivery_method_id {
            let package = match user_country_code.clone() {
                None => get_select_package_v1(context, delivery_method_id)?,
                Some(user_country_code) => {
                    get_selected_package_by_product(context, cart_item.product_id, user_country_code, delivery_method_id)?
                }
            };
            packages.push((cart_item, package));
        }
    }

    Ok(packages.into_iter().collect())
}

pub fn get_delivery_info(packages: HashMap<ProductId, AvailablePackageForUser>) -> HashMap<ProductId, DeliveryInfo> {
    packages
        .into_iter()
        .map(|(product_id, package)| {
            let element = available_packages::get_delivery_info(package);

            (product_id, element)
        })
        .collect::<HashMap<ProductId, DeliveryInfo>>()
}

pub fn get_product_info(context: &Context, cart_items: &HashSet<CartItem>) -> FieldResult<HashMap<ProductId, ProductInfo>> {
    cart_items
        .iter()
        .map(|cart_item| {
            product_module::get_product(context, cart_item.product_id).and_then(|product| {
                let product_info = ProductInfo::from(product);

                Ok((cart_item.product_id, product_info))
            })
        })
        .collect()
}

fn get_select_package_v1(context: &Context, delivery_method: DeliveryMethodId) -> FieldResult<AvailablePackageForUser> {
    match delivery_method {
        DeliveryMethodId::Package { .. } => Err(FieldError::new(
            "Could not get selected package.",
            graphql_value!({ "code": 100, "details": { "Invalid order. Please create a new order." }}),
        )),
        DeliveryMethodId::ShippingPackage { id: shipping_id } => get_available_package_for_user_by_id_v1(context, shipping_id),
        _ => Err(FieldError::new(
            "Could not get selected package.",
            graphql_value!({ "code": 100, "details": { "Delivery method is not supported." }}),
        )),
    }
}

fn get_selected_package_by_product(
    context: &Context,
    product_id: ProductId,
    user_country_code: String,
    delivery_method: DeliveryMethodId,
) -> FieldResult<AvailablePackageForUser> {
    let product = product_module::try_get_product(context, product_id)?.ok_or(FieldError::new(
        "Could not get selected package.",
        graphql_value!({ "code": 100, "details": { "Product not found." }}),
    ))?;
    get_select_package(context, product.base_product_id, user_country_code, delivery_method)
}

fn get_select_package(
    context: &Context,
    base_product_id: BaseProductId,
    user_country_code: String,
    delivery_method: DeliveryMethodId,
) -> FieldResult<AvailablePackageForUser> {
    let shipping_id = match delivery_method {
        DeliveryMethodId::Package { .. } => Err(FieldError::new(
            "Could not get selected package.",
            graphql_value!({ "code": 100, "details": { "Invalid order. Please create a new order." }}),
        ))?,
        DeliveryMethodId::ShippingPackage { id: shipping_id } => shipping_id,
        _ => Err(FieldError::new(
            "Could not get selected package.",
            graphql_value!({ "code": 100, "details": { "Delivery method is not supported." }}),
        ))?,
    };

    let (_, package) = get_available_package_for_user_with_price(
        context,
        base_product_id,
        shipping_id,
        user_country_code.as_str(),
        "Could not get selected package.",
    )?;

    Ok(package)
}

pub fn validate_select_package(cart_product: &CartItem, package: &AvailablePackageForUser) -> FieldResult<()> {
    if cart_product.store_id != package.store_id {
        return Err(FieldError::new(
            "Selected package is not valid.",
            graphql_value!({ "code": 100, "details": { "The selected package is not found in the store." }}),
        ));
    }

    Ok(())
}
