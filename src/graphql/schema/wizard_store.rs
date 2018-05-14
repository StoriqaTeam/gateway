//! File containing wizard store object of graphql schema
use futures::Future;
use hyper::Method;
use juniper::FieldResult;
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::Language;

use super::*;
use graphql::context::Context;
use graphql::models::*;

graphql_object!(WizardStore: Context as "WizardStore" |&self| {
    description: "Store's wizard info."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        ID::new(Service::Stores, Model::WizardStore, self.id).to_string().into()
    }

    field raw_id() -> &i32 as "Unique int id"{
        &self.id
    }

    field name() -> &Option<String> as "Full Name" {
        &self.name
    }

    field short_description() -> &Option<String> as "Short description" {
        &self.short_description
    }

    field slug() -> &Option<String> as "Slug" {
        &self.slug
    }

    field address() -> &Option<String> as "Address" {
        &self.address
    }

    field country() -> &Option<String> as "Country" {
        &self.country
    }

    field default_language() -> &Option<Language> as "Default language" {
        &self.default_language
    }

    field store_id() -> &Option<i32> as "Store raw id" {
        &self.store_id
    }

    field moderator_comment(&executor) -> FieldResult<Option<ModeratorStoreComments>> as "Fetches moderator comment by id." {
        if let Some(ref store_id) = &self.store_id {
            let context = executor.context();

            let url = format!(
                "{}/{}/{}",
                &context.config.service_url(Service::Stores),
                Model::ModeratorStoreComment.to_url(),
                store_id
            );

            context.request::<ModeratorStoreComments>(Method::Get, url, None)
                .wait()
                .map(|u| Some(u))
        } else {
            Ok(None)
        }
    }

});
