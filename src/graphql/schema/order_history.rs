//! File containing PageInfo object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::OrderState;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(OrderHistoryItem: Context as "OrderHistoryItem" |&self| {
    description: "Order history item info."

    field state() -> &OrderState as "Order State"{
        &self.state
    }

    field order_id() -> &str as "Order id"{
        &self.parent
    }

    field committer() -> &i32 as "User int id"{
        &self.committer.0
    }

    field user(&executor) -> FieldResult<Option<User>> as "User" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            self.committer);

        context.request::<Option<User>>(Method::Get, url, None)
            .wait()
    }

    field committed_at() -> String as "Committed at time" {
        self.committed_at.to_rfc3339()
    }

    field comment() -> &Option<String> as "Comment" {
        &self.comment
    }

});
