//! File containing Coupon object of graphql schema
use chrono::prelude::*;
use futures::Future;
use graphql::context::Context;
use graphql::models::*;
use hyper::Method;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};

use serde_json;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_types::{CouponCode, CouponId, StoreId};

graphql_object!(Coupon: Context as "Coupon" |&self| {
    description: "Coupon info."

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::Coupon, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 as "Raw numeric id"{
         &self.id.0
    }

    field code() -> &str as "Coupon id" {
        &self.code.0
    }

    field title() -> &str as "Title" {
        &self.title
    }

    field store_id() -> GraphqlID as "Base64 Store id"{
        ID::new(Service::Stores, Model::Store, self.store_id.0).to_string().into()
    }

    field scope() -> CouponScope as "Coupon scope" {
        self.scope
    }

    field percent() -> i32 as "Percent" {
        self.percent
    }

    field quantity() -> i32 as "Quantity" {
        self.quantity
    }

    field used_quantity() -> i32 as "Count activations" {
        0 // TODO: get from stores
    }

    field expired_at() -> Option<String> as "Expired at" {
        self.expired_at
            .map(DateTime::<Utc>::from)
            .map(|datetime| datetime.to_rfc3339())
    }

    field is_active() -> bool as "Is active" {
        self.is_active
    }

    field created_at() -> String as "Created at" {
        let datetime: DateTime<Utc> = self.created_at.into();
        datetime.to_rfc3339()
    }

    field updated_at() -> String as "Updated at" {
        let datetime: DateTime<Utc> = self.updated_at.into();
        datetime.to_rfc3339()
    }

    field base_products(&executor) -> FieldResult<Vec<BaseProduct>> as "Base products coupon can be applied to" {
        let context = executor.context();
        let url = format!("{}/{}/{}/base_products",
            context.config.service_url(Service::Stores),
            Model::Coupon.to_url(),
            self.id);
        context.request::<Vec<BaseProduct>>(Method::Get, url, None).wait()
    }

});

pub fn validate_coupon_by_code(context: &Context, coupon_code: CouponCode, store_id: StoreId) -> FieldResult<()> {
    // Validate coupon
    let url = format!(
        "{}/{}/validate/code",
        context.config.service_url(Service::Stores),
        Model::Coupon.to_url()
    );

    let search_code = CouponsSearchCodePayload {
        code: coupon_code,
        store_id: store_id,
    };

    let body = serde_json::to_string(&search_code)?;

    let check_result = context
        .request::<Option<CouponValidate>>(Method::Post, url, Some(body))
        .wait()?
        .ok_or_else(|| {
            FieldError::new(
                "Coupon not found",
                graphql_value!({ "code": 400, "details": { "coupon not found" }}),
            )
        })?;

    check_result.validate()
}

pub fn validate_coupon(context: &Context, coupon_id: CouponId) -> FieldResult<()> {
    // Validate coupon
    let url = format!(
        "{}/{}/{}/validate",
        context.config.service_url(Service::Stores),
        Model::Coupon.to_url(),
        coupon_id,
    );

    let check_result = context
        .request::<Option<CouponValidate>>(Method::Get, url, None)
        .wait()?
        .ok_or_else(|| {
            FieldError::new(
                "Coupon not found",
                graphql_value!({ "code": 400, "details": { "coupon not found" }}),
            )
        })?;

    check_result.validate()
}

pub fn get_coupon_by_code(context: &Context, coupon_code: CouponCode, store_id: StoreId) -> FieldResult<Coupon> {
    let url = format!(
        "{}/{}/search/code",
        context.config.service_url(Service::Stores),
        Model::Coupon.to_url()
    );

    let search_code = CouponsSearchCodePayload {
        code: coupon_code,
        store_id: store_id,
    };

    let body = serde_json::to_string(&search_code)?;

    context
        .request::<Option<Coupon>>(Method::Post, url, Some(body))
        .wait()?
        .ok_or_else(|| {
            FieldError::new(
                "Coupon not found",
                graphql_value!({ "code": 400, "details": { "coupon not found" }}),
            )
        })
}

pub fn get_coupon(context: &Context, coupon_id: CouponId) -> FieldResult<Coupon> {
    try_get_coupon(context, coupon_id)?.ok_or_else(|| {
        FieldError::new(
            "Coupon not found",
            graphql_value!({ "code": 400, "details": { "coupon not found" }}),
        )
    })
}

pub fn try_get_coupon(context: &Context, coupon_id: CouponId) -> FieldResult<Option<Coupon>> {
    let url = format!(
        "{}/{}/{}",
        context.config.service_url(Service::Stores),
        Model::Coupon.to_url(),
        coupon_id,
    );

    context.request::<Option<Coupon>>(Method::Get, url, None).wait()
}
