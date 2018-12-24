//! File containing Category object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Translation;
use stq_types::CategoryId;

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
                // TODO: use `try_get_category`
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

    field slug() -> &str {
        &self.slug
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

graphql_object!(CategoryWithProducts: Context as "CategoryWithProducts" |&self| {
    description: "Category info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::CategoryWithProducts, self.0.id.0).to_string().into()
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

    field parent(&executor) -> FieldResult<Option<CategoryWithProducts>> as "Parent category" {
        match self.0.parent_id.as_ref() {
            Some(parent_id) => {
                let context = executor.context();
                try_get_category(context, *parent_id).map(|c| c.map(CategoryWithProducts))
            },
            None => Ok(None)
        }
    }

    field level() -> &i32 as "Level" {
        &self.0.level
    }

    field children() -> Vec<CategoryWithProducts> as "Children categories" {
        self.0.children.iter().cloned().map(CategoryWithProducts).collect()
    }

    field get_attributes(&executor) -> &[Attribute] as "Fetches category attributes." {
        &self.0.attributes
    }

    field slug() -> &str {
        &self.0.slug
    }
});

pub fn run_replace_category(context: &Context, payload: CategoryReplaceInput) -> FieldResult<Vec<BaseProduct>> {
    let url = format!(
        "{}/{}/replace_category",
        context.config.service_url(Service::Stores),
        Model::BaseProduct.to_url(),
    );

    let body: String = serde_json::to_string(&payload)?.to_string();

    context.request::<Vec<BaseProduct>>(Method::Post, url, Some(body)).wait()
}

pub fn categories_with_products(context: &Context) -> FieldResult<Option<CategoryWithProducts>> {
    let url = format!(
        "{}/{}/with_products",
        context.config.service_url(Service::Stores),
        Model::Category.to_url()
    );

    context
        .request::<Category>(Method::Get, url, None)
        .wait()
        .map(CategoryWithProducts)
        .map(Some)
}

pub fn try_get_category(context: &Context, category_id: CategoryId) -> FieldResult<Option<Category>> {
    let url = format!(
        "{}/{}/{}",
        context.config.service_url(Service::Stores),
        Model::Category.to_url(),
        category_id
    );

    context.request::<Option<Category>>(Method::Get, url, None).wait()
}
