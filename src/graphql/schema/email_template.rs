//! File containing node object of graphql schema
use futures::Future;
use hyper::Method;

use juniper::FieldResult;

use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;
use stq_routes::model::Model;

graphql_object!(EmailTemplate: Context as "EmailTemplate" |&self| {
    description: "Email template."

    field users_template_order_update_state(&executor) -> FieldResult<String> as "Email messages template for user order update state event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::User.to_url(),
            "template-order-update-state");

        context.request::<String>(Method::Get, url, None)
            .wait()
    }

    field stores_template_order_update_state(&executor) -> FieldResult<String> as "Email messages template for store order update state event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::Store.to_url(),
            "template-order-update-state");

        context.request::<String>(Method::Get, url, None)
            .wait()
    }

    field users_template_order_create(&executor) -> FieldResult<String> as "Email messages template for user order create event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::User.to_url(),
            "template-order-create");

        context.request::<String>(Method::Get, url, None)
            .wait()
    }

    field stores_template_order_create(&executor) -> FieldResult<String> as "Email messages template for store order create event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::Store.to_url(),
            "template-order-create");

        context.request::<String>(Method::Get, url, None)
            .wait()
    }

    field template_email_verification(&executor) -> FieldResult<String> as "Email messages template for user email verification event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::User.to_url(),
            "template-email-verification");

        context.request::<String>(Method::Get, url, None)
            .wait()
    }

    field template_apply_email_verification(&executor) -> FieldResult<String> as "Email messages template for user apply email verification event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::User.to_url(),
            "template-apply-email-verification");

        context.request::<String>(Method::Get, url, None)
            .wait()
    }

    field template_password_reset(&executor) -> FieldResult<String> as "Email messages template for user password reset event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::User.to_url(),
            "template-password-reset");

        context.request::<String>(Method::Get, url, None)
            .wait()
    }

    field template_apply_password_reset(&executor) -> FieldResult<String> as "Email messages template for user apply password reset event" {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Notifications),
            Model::User.to_url(),
            "template-apply-password-reset");

        context.request::<String>(Method::Get, url, None)
            .wait()
    }

});
