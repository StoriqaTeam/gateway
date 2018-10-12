//! File containing product object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::{AttributeType, Translation};

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Attribute: Context as "Attribute" |&self| {
    description: "Attribute's info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::Attribute, self.id.0).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id.0
    }

    field name() -> &[Translation] as "Full Name" {
        &self.name
    }

    field value_type() -> &AttributeType as "Attribute Type" {
        &self.value_type
    }

    field meta_field() -> &Option<AttributeMetaField> as "Meta field of product" {
        &self.meta_field
    }
});

graphql_object!(AttrValue: Context as "AttributeValue" |&self| {
    description: "Product variant attributes with values."

    field attribute(&executor) -> FieldResult<Option<Attribute>> as "Attribute" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Attribute.to_url(),
            self.attr_id.0);
        context.request::<Option<Attribute>>(Method::Get, url, None)
            .wait()
    }

    field attr_id() -> &i32 as "Attribute id" {
        &self.attr_id.0
    }

    field value() -> &str as "Attribute value of product variant" {
        &self.value.0
    }

    field meta_field() -> &Option<String> as "Meta field of product attribute value" {
        &self.meta_field
    }
});

graphql_object!(AttributeFilter: Context as "AttributeFilter" |&self| {
    description: "Attribute Filter"

    field attribute(&executor) -> FieldResult<Option<Attribute>> as "Attribute" {
        let context = executor.context();
        let url = format!("{}/{}/{}",
            context.config.service_url(Service::Stores),
            Model::Attribute.to_url(),
            self.id);
        context.request::<Option<Attribute>>(Method::Get, url, None)
            .wait()
    }

    field equal() -> &Option<EqualFilter> as "Values to be equal" {
        &self.equal
    }

    field range() -> &Option<RangeFilter> as "Range values to compare" {
        &self.range
    }
});

graphql_object!(AttributeMetaField: Context as "AttributeMetaField" |&self| {
    description: "Attribute Meta Field"

    field values() -> &Option<Vec<String>> as "Possible values of attribute" {
        &self.values
    }

    field translated_values() -> Vec<TranslatedValue> as "Possible values of attribute with translation" {
        let vals = self.translated_values.clone().unwrap_or_default();
        vals.into_iter().map(|v| TranslatedValue {translations: v}).collect()
    }

    field ui_element() -> &UIType as "UI element type" {
        &self.ui_element
    }
});

#[derive(GraphQLObject, Deserialize, Clone, Debug, PartialEq)]
#[graphql(description = "Value with translation")]
pub struct TranslatedValue {
    #[graphql(description = "Translated value")]
    pub translations: Vec<Translation>,
}
