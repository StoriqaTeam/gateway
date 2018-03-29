//! File containing product object of graphql schema
use juniper::ID as GraphqlID;
use juniper::{FieldResult};
use hyper::Method;
use futures::Future;

use stq_static_resources::Translation;
use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;
use super::*;

graphql_object!(Attribute: Context as "Attribute" |&self| {
    description: "Attribute's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Unique id"{
        ID::new(Service::Stores, Model::Attribute, self.id).to_string().into()
    }

    field raw_id() -> GraphqlID as "Unique id"{
        self.id.to_string().into()
    }

    field name() -> Vec<Translation> as "Full Name" {
        self.name.clone()
    }
    
    field value_type() -> AttributeType as "Attribute Type" {
        self.value_type.clone()
    }

    field meta_field() -> Option<AttributeMetaField> as "Meta field of product" {
        self.meta_field.clone()
    }
});


graphql_object!(AttrValue: Context as "AttributeValue" |&self| {
    description: "Product variant attributes with values."

    field attribute(&executor) -> FieldResult<Option<Attribute>> as "Attribute" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Attribute.to_url(),
            self.attr_id);
        context.http_client.request_with_auth_header::<Attribute>(Method::Get, url, None, context.user.as_ref().map(|t| t.to_string()))
                    .or_else(|err| Err(err.into_graphql()))
                    .wait()
                    .map(|u| Some(u))
    }
    
    field value() -> String as "Attribute value of product variant" {
        self.value.clone()
    }

    field meta_field() -> Option<String> as "Meta field of product attribute value" {
        self.meta_field.clone()
    }
});
