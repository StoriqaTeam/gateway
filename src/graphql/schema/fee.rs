//! File containing Fee object of graphql schema
use graphql::context::Context;
use graphql::models::*;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Currency;

graphql_object!(Fee: Context as "Fee" |&self| {
    description: "Fee info."

    field id() -> GraphqlID as "Base64 Unique id" {
        ID::new(Service::Billing, Model::Fee, *self.id.inner()).to_string().into()
    }

    field order_id() -> String as "Order id" {
        self.order_id.to_string()
    }

    field amount() -> &f64 as "Amount" {
        &self.amount
    }

    field status() -> &FeeStatus as "Status" {
        &self.status
    }

    field currency() -> &Currency as " Currency" {
        &self.currency
    }

    field charge_id() -> Option<String> as "Charge id" {
        self.charge_id.as_ref().map(|v| v.0.clone())
    }
});
