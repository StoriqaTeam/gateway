//! File containing node object of graphql schema
use futures::Future;
use graphql::context::Context;
use graphql::models::*;
use hyper::Method;
use juniper::FieldResult;

use stq_routes::service::Service;
use stq_static_resources::TemplateVariant;

graphql_object!(EmailTemplate: Context as "EmailTemplate" |&self| {
    description: "Email template."

    field template(&executor, variant: TemplateVariant) -> FieldResult<String> as "Email messages template" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            "templates",
            variant);

        context.request::<String>(Method::Get, url, None)
            .wait()
    }

});
