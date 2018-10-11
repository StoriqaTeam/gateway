//! File containing product object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(CustomAttribute: Context as "CustomAttribute" |&self| {
    description: "custom attribute's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::CustomAttribute, self.id).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id
    }

    field attribute_id() -> &i32 as "Unique int attribute id"{
        &self.attribute_id
    }

    field attribute(&executor) -> FieldResult<Option<Attribute>> as "Attribute" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Attribute.to_url(),
            self.attribute_id);

        context.request::<Option<Attribute>>(Method::Get, url, None)
            .wait()
    }

    field base_product_id() -> &i32 as "Unique int base product id"{
        &self.base_product_id.0
    }
});
