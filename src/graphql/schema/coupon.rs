//! File containing Coupon object of graphql schema
use chrono::prelude::*;
use futures::Future;
use graphql::context::Context;
use graphql::models::*;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;

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
