//! File containing wizard store object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Currency;
use stq_types::{BaseProductId, ShippingId};

use graphql::context::Context;
use graphql::models::*;
use graphql::schema::base_product as base_product_module;

graphql_object!(AvailablePackages: Context as "AvailablePackages" |&self| {
    description: "Available Packages info."

    field company_package_id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Delivery, Model::CompanyPackage, self.id.0).to_string().into()
    }

    field company_package_raw_id() -> &i32 as "Int company package id"{
        &self.id.0
    }

    field name() -> &str as "Available package name"{
        &self.name
    }

    field logo() -> &str as "Company logo"{
        &self.logo
    }

    field deliveries_to() -> &[Country] as "Deliveries to Countries." {
        &self.deliveries_to
    }

    field currency() -> Currency as "Company currency" {
        self.currency
    }

});

graphql_object!(AvailablePackagesOutput: Context as "AvailablePackagesOutput" |&self| {
    description: "Available Packages info."

    field local() -> &[AvailablePackages] as "Local packages"{
        &self.local
    }

    field international() -> &[AvailablePackages] as "International packages"{
        &self.international
    }

});

graphql_object!(AvailablePackageForUser: Context as "AvailablePackageForUser" |&self| {
    description: "Available Packages info."

    field id() -> GraphqlID as "Base64 Unique id" {
        ID::new(Service::Delivery, Model::AvailablePackageForUser, self.shipping_id.0).to_string().into()
    }

    field deprecated "use id" company_package_id() -> GraphqlID as "Base64 Unique id for company package"{
        ID::new(Service::Delivery, Model::CompanyPackage, self.id.0).to_string().into()
    }

    field company_package_raw_id() -> &i32 as "Int company package id"{
        &self.id.0
    }

    field shipping_id() -> &i32 as "Int shipping id" {
        &self.shipping_id.0
    }

    field name() -> &str as "Available package name"{
        &self.name
    }

    field logo() -> &str as "Company logo"{
        &self.logo
    }

    field price() -> f64 as "Package price." {
        self.price.0
    }
});

graphql_object!(AvailableShippingForUser: Context as "AvailableShippingForUser" |&self| {
    description: "Available Packages info."

    field packages() -> &[AvailablePackageForUser] as "Available Packages For Users"{
        &self.packages
    }

    field pickups() -> &Option<PickupsOutput> as "Pickups"{
        &self.pickups
    }

});

pub fn get_available_package_for_user_by_id_v1(context: &Context, shipping_id: ShippingId) -> FieldResult<AvailablePackageForUser> {
    let url = format!(
        "{}/{}/by_shipping_id/{}",
        context.config.service_url(Service::Delivery),
        Model::AvailablePackageForUser.to_url(),
        shipping_id,
    );

    context
        .request::<Option<AvailablePackageForUser>>(Method::Get, url, None)
        .wait()?
        .ok_or_else(|| {
            FieldError::new(
                "Could not find AvailablePackageForUser.",
                graphql_value!({ "code": 100, "details": { "Select available package not found" }}),
            )
        })
}

pub fn try_get_available_package_for_user_by_id(
    context: &Context,
    shipping_id: ShippingId,
    delivery_from: &str,
    delivery_to: &str,
    volume: u32,
    weight: u32,
) -> FieldResult<Option<AvailablePackageForUser>> {
    let url = format!(
        "{}/v2/{}/by_shipping_id/{}?delivery_from={}&delivery_to={}&volume={}&weight={}",
        context.config.service_url(Service::Delivery),
        Model::AvailablePackageForUser.to_url(),
        shipping_id,
        delivery_from,
        delivery_to,
        volume,
        weight,
    );

    context.request::<Option<AvailablePackageForUser>>(Method::Get, url, None).wait()
}

pub fn get_available_package_for_user_by_id(
    context: &Context,
    shipping_id: ShippingId,
    delivery_from: &str,
    delivery_to: &str,
    volume: u32,
    weight: u32,
    context_err_msg: &str,
) -> FieldResult<AvailablePackageForUser> {
    try_get_available_package_for_user_by_id(context, shipping_id, delivery_from, delivery_to, volume, weight)?.ok_or_else(|| {
        FieldError::new(
            context_err_msg,
            graphql_value!({ "code": 100, "details": { "Select available package not found" }}),
        )
    })
}

pub fn get_delivery_info(package: AvailablePackageForUser) -> DeliveryInfo {
    DeliveryInfo {
        company_package_id: package.id,
        shipping_id: package.shipping_id,
        name: package.name,
        logo: package.logo,
        price: package.price.0,
    }
}

pub fn try_get_available_package_for_user_with_price(
    context: &Context,
    base_product_id: BaseProductId,
    shipping_id: ShippingId,
    user_country_code: &str,
    context_err_msg: &str,
) -> FieldResult<(BaseProductShippingDetails, Option<AvailablePackageForUser>)> {
    let shipping_details = base_product_module::get_base_product_shipping_details(context, base_product_id, context_err_msg)?;
    let package = try_get_available_package_for_user_by_id(
        context,
        shipping_id,
        shipping_details.delivery_from.as_str(),
        user_country_code,
        shipping_details.measurements.volume_cubic_cm,
        shipping_details.measurements.weight_g,
    )?;

    if let Some(ref package) = package {
        if shipping_details.store_id != package.store_id {
            Err(FieldError::new(
                context_err_msg,
                graphql_value!({ "code": 100, "details": { "The selected package is not found in the store." }}),
            ))?;
        }
    };

    Ok((shipping_details, package))
}

pub fn get_available_package_for_user_with_price(
    context: &Context,
    base_product_id: BaseProductId,
    shipping_id: ShippingId,
    user_country_code: &str,
    context_err_msg: &str,
) -> FieldResult<(BaseProductShippingDetails, AvailablePackageForUser)> {
    let (shipping_details, package) =
        try_get_available_package_for_user_with_price(context, base_product_id, shipping_id, user_country_code, context_err_msg)?;

    match package {
        None => Err(FieldError::new(
            context_err_msg,
            graphql_value!({ "code": 100, "details": { "Available package for user not found." }}),
        )),
        Some(package) => Ok((shipping_details, package)),
    }
}
