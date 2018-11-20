//! File containing product object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::{AttributeType, Translation};
use stq_types::{AttributeId, AttributeValueId};

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

    field meta_field(&executor) -> FieldResult<Option<AttributeMetaField>> as "Meta field of product" {
        get_attribute_meta_field(executor.context(), self.id, self.meta_field.clone())
    }

    field values(&executor) -> FieldResult<Option<Vec<AttributeValue>>> as "Attribute values" {
        get_attribute_values(executor.context(), self.id)
    }
});

graphql_object!(ProdAttrValue: Context as "ProdAttrValue" |&self| {
    description: "Product variant attributes with values."

    field attribute(&executor) -> FieldResult<Option<Attribute>> as "Attribute" {
        let context = executor.context();

        try_get_attribute(context, self.attr_id)
    }

    field attr_id() -> &i32 as "Attribute id" {
        &self.attr_id.0
    }

    field attribute_value_id() -> Option<i32> as "Attribute value id" {
        self.attr_value_id.map(|id| id.0)
    }

    field attribute_value(&executor) -> FieldResult<Option<AttributeValue>>  as "Attribute value" {
        match self.attr_value_id {
            Some(attr_value_id) => get_attribute_value(executor.context(), attr_value_id).map(Some),
            None => Ok(None)
        }
    }

    field deprecated "use attribute_value.code" value() -> &str as "Attribute value of product variant" {
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

        try_get_attribute(context, AttributeId(self.id))
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

    field deprecated "use attribute values" values() -> &Option<Vec<String>> as "Possible values of attribute" {
        &self.values
    }

    field deprecated "use attribute values" translated_values() -> Vec<TranslatedValue> as "Possible values of attribute with translation" {
        let vals = self.translated_values.clone().unwrap_or_default();
        vals.into_iter().map(|v| TranslatedValue {translations: v}).collect()
    }

    field ui_element() -> &UIType as "UI element type" {
        &self.ui_element
    }
});

graphql_object!(AttributeValue: Context as "AttributeValue" |&self| {
    description: "Attribute Value"

    field attribute(&executor) -> FieldResult<Option<Attribute>> as "Attribute" {
        let context = executor.context();

        try_get_attribute(context, self.attr_id)
    }

    field raw_id() -> &i32 as "Raw attribute value id" {
        &self.id.0
    }

    field attr_raw_id() -> &i32 as "Raw attribute id" {
        &self.attr_id.0
    }

    field code() -> &str as "Value code" {
        &self.code.0
    }

    field translation() -> Vec<Translation> as "Possible translations of value" {
        self.translations.clone().unwrap_or_default()
    }
});

#[derive(GraphQLObject, Deserialize, Clone, Debug, PartialEq)]
#[graphql(description = "Value with translation")]
pub struct TranslatedValue {
    #[graphql(description = "Translated value")]
    pub translations: Vec<Translation>,
}

fn get_attribute_values(context: &Context, attribute_id: AttributeId) -> FieldResult<Option<Vec<AttributeValue>>> {
    let url = format!(
        "{}/{}/{}/{}",
        context.config.service_url(Service::Stores),
        Model::Attribute.to_url(),
        attribute_id,
        Model::AttributeValue.to_url(),
    );

    context.request::<Option<Vec<AttributeValue>>>(Method::Get, url, None).wait()
}

fn get_attribute_value(context: &Context, attribute_value_id: AttributeValueId) -> FieldResult<AttributeValue> {
    let url = format!(
        "{}/{}/{}/{}",
        context.config.service_url(Service::Stores),
        Model::Attribute.to_url(),
        Model::AttributeValue.to_url(),
        attribute_value_id
    );

    context.request::<AttributeValue>(Method::Get, url, None).wait()
}

fn get_attribute_meta_field(
    context: &Context,
    attribute_id: AttributeId,
    meta: Option<AttributeMetaField>,
) -> FieldResult<Option<AttributeMetaField>> {
    let attribute_values = get_attribute_values(context, attribute_id)?;
    let (codes, translations) = attribute_values
        .map(|values| {
            let mut codes = Vec::with_capacity(values.len());
            let mut translations = Vec::with_capacity(values.len());
            for value in values {
                codes.push(value.code.0);
                if let Some(value_translations) = value.translations {
                    translations.push(value_translations);
                }
            }
            (
                Some(codes).filter(|codes| !codes.is_empty()),
                Some(translations).filter(|t| !t.is_empty()),
            )
        }).unwrap_or((None, None));
    Ok(meta.map(|meta| AttributeMetaField {
        ui_element: meta.ui_element,
        values: codes,
        translated_values: translations,
    }))
}

fn try_get_attribute(context: &Context, attribute_id: AttributeId) -> FieldResult<Option<Attribute>> {
    let url = format!(
        "{}/{}/{}",
        context.config.service_url(Service::Stores),
        Model::Attribute.to_url(),
        attribute_id.0
    );

    context.request::<Option<Attribute>>(Method::Get, url, None).wait()
}
