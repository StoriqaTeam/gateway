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

    field attribute_id() -> &i32 as "Unique int custom attribute id"{
        &self.attribute_id
    }

    field base_product_id() -> &i32 as "Unique int base product id"{
        &self.base_product_id.0
    }
});

graphql_object!(CustomAttributeValue: Context as "CustomAttributeValue" |&self| {
    description: "Product variant custom attributes with values."

    field custom_attribute(&executor) -> FieldResult<Option<CustomAttribute>> as "CustomAttribute" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::CustomAttribute.to_url(),
            self.custom_attribute_id);
        context.request::<Option<CustomAttribute>>(Method::Get, url, None)
            .wait()
    }

    field custom_attribute_id() -> &i32 as "Custom attribute id" {
        &self.custom_attribute_id
    }

    field value() -> &str as "Custom attribute value of product variant" {
        &self.value
    }

});
