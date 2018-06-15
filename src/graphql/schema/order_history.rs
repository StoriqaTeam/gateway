//! File containing PageInfo object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;

use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(OrderHistoryItem: Context as "OrderHistoryItem" |&self| {
    description: "Order history item info."

    field status() -> &OrderStatus as "Order Status"{
        &self.status
    }

    field user_id() -> &i32 as "User int id"{
        &self.user_id
    }

    field user(&executor) -> FieldResult<Option<User>> as "User" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Users),
            Model::User.to_url(),
            self.user_id);

        context.request::<Option<User>>(Method::Get, url, None)
            .wait()
    }

    field creation_time() -> &str as "Creation time" {
        &self.creation_time
    }

    field comments() -> &Option<String> as "Comments" {
        &self.comments
    }

});
