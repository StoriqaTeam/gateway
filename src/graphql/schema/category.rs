//! File containing Category object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Translation;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Category: Context as "Category" |&self| {
    description: "Category info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::Category, self.id.0).to_string().into()
    }

    field raw_id() -> i32 as "Unique int id"{
        self.id.0
    }

    field name() -> &[Translation] as "Full Name" {
        &self.name
    }

    field meta_field() -> &Option<String> as "Meta field" {
        &self.meta_field
    }

    field parent_id() -> Option<i32> as "Parent id" {
        self.parent_id.map(|id| id.0)
    }

    field parent(&executor) -> FieldResult<Option<Category>> as "Parent category" {
        match self.parent_id.as_ref() {
            Some(parent_id) => {
                let context = executor.context();
                let url = format!("{}/{}/{}",
                context.config.service_url(Service::Stores),
                Model::Category.to_url(),
                parent_id.0);

                context.request::<Option<Category>>(Method::Get, url, None)
                    .wait()
            },
            None => Ok(None)
        }
    }

    field level() -> &i32 as "Level" {
        &self.level
    }

    field children() -> &[Category] as "Children categories" {
        &self.children
    }

    field get_attributes(&executor) -> &[Attribute] as "Fetches category attributes." {
        &self.attributes
    }
});

graphql_object!(SearchCategory: Context as "SearchCategory" |&self| {
    description: "Search Category info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::SearchCategory, self.0.id.0).to_string().into()
    }

    field raw_id() -> i32 as "Unique int id"{
        self.0.id.0
    }

    field name() -> &[Translation] as "Full Name" {
        &self.0.name
    }

    field meta_field() -> &Option<String> as "Meta field" {
        &self.0.meta_field
    }

    field parent_id() -> Option<i32> as "Parent id" {
        self.0.parent_id.map(|id| id.0)
    }

    field level() -> &i32 as "Level" {
        &self.0.level
    }

    field children() -> Vec<SearchCategory> as "Children categories" {
        self.0.children.clone().into_iter().map(SearchCategory).collect::<Vec<SearchCategory>>()
    }

});
