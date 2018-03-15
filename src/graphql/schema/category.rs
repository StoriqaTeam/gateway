//! File containing Category object of graphql schema
use juniper::ID as GraphqlID;
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Translation;
use juniper::FieldResult;
use hyper::Method;
use futures::Future;

use graphql::context::Context;
use graphql::models::*;

graphql_object!(Category: Context as "Category" |&self| {
    description: "Category info."

    field id() -> GraphqlID as "Unique id"{
        ID::new(Service::Stores, Model::Category, self.id).to_string().into()
    }

    field raw_id() -> GraphqlID as "Unique id"{
        self.id.to_string().into()
    }

    field name() -> Vec<Translation> as "Full Name" {
        self.name.clone()
    }

    field meta_field() -> Option<String> as "Meta field" {
        self.meta_field.clone()
    }

    field children() -> Vec<Category> as "Children categories" {
        self.children.clone()
    }

    field get_attributes(&executor) -> FieldResult<Vec<Attribute>> as "Fetches category attributes." {
        let context = executor.context();
        let url = format!("{}/{}/{}/attributes",
            context.config.service_url(Service::Stores),
            Model::Category.to_url(),
            self.id
            );

        context.http_client.request_with_auth_header::<Vec<Attribute>>(Method::Get, url, None, context.user.as_ref().map(|t| t.to_string()))
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }
});
