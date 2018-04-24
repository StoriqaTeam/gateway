//! File containing query object of graphql schema
use std::str::FromStr;

use futures::Future;
use hyper::Method;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};
use stq_routes::model::Model;
use stq_routes::service::Service;
use stq_static_resources::currency::{Currency, CurrencyGraphQl};
use stq_static_resources::{Language, LanguageGraphQl};

use super::*;
use graphql::context::Context;
use graphql::models::*;

pub const QUERY_NODE_ID: i32 = 1;

pub struct Query;

graphql_object!(Query: Context |&self| {

    description: "Top level query.

    Remote mark

    Some fields are marked as `Remote`. That means that they are
    part of microservices and their fetching can fail.
    In this case null will be returned (even if o/w
    type signature declares not-null type) and corresponding errors
    will be returned in errors section. Each error is guaranteed
    to have a `code` field and `details field`.

    Codes:
    - 100 - microservice responded,
    but with error http status. In this case `details` is guaranteed
    to have `status` field with http status and
    probably some additional details.

    - 200 - there was a network error while connecting to microservice.

    - 300 - there was a parse error - that usually means that
    graphql couldn't parse api json response
    (probably because of mismatching types on graphql and microservice)
    or api url parse failed.

    - 400 - Unknown error."

    interfaces: [&Node]

    field id() -> GraphqlID as "Base64 Unique id"{
        QUERY_NODE_ID.to_string().into()
    }

    field apiVersion() -> &str as "Current api version." {
        "1.0"
    }

    field static_node_id() -> StaticNodeIds as "Static node id dictionary." {
        StaticNodeIds{}
    }

    field me(&executor) -> FieldResult<Option<User>> as "Fetches viewer for users." {
        let context = executor.context();
        let url = format!("{}/{}/current",
            context.config.service_url(Service::Users),
            Model::User.to_url());
        context.request::<User>(Method::Get, url, None)
                    .wait()
                    .map(|u| Some(u))
    }

    field node(&executor, id: GraphqlID as "Base64 Id of a node.") -> FieldResult<Option<Node>> as "Fetches graphql interface node by Base64 id."  {
        let context = executor.context();
        if id.to_string() == QUERY_NODE_ID.to_string() {
             Ok(Some(Node::Query(Query{})))
        } else {
            let identifier = ID::from_str(&*id)?;
            match (&identifier.service, &identifier.model) {
                (&Service::Users, &Model::User) => {
                                context.request::<User>(Method::Get, identifier.url(&context.config), None)
                                    .map(|res| Node::User(res))
                                    .wait()
                                    .map(|u| Some(u))
                },
                (&Service::Users, _) => {
                                Err(FieldError::new(
                                    "Could not get model from users microservice.",
                                    graphql_value!({ "internal_error": "Unknown model" })
                                ))
                },
                (&Service::Stores, &Model::Store) => {
                                context.request::<Store>(Method::Get, identifier.url(&context.config), None)
                                    .map(|res| Node::Store(res))
                                    .wait()
                                    .map(|u| Some(u))
                },
                (&Service::Stores, &Model::Product) => {
                                context.request::<Product>(Method::Get, identifier.url(&context.config), None)
                                    .map(|res| Node::Product(res))
                                    .wait()
                                    .map(|u| Some(u))
                },
                (&Service::Stores, &Model::BaseProduct) => {
                                context.request::<BaseProduct>(Method::Get, identifier.url(&context.config), None)
                                    .map(|res| Node::BaseProduct(res))
                                    .wait()
                                    .map(|u| Some(u))
                },
                (&Service::Stores, &Model::Category) => {
                                context.request::<Category>(Method::Get, identifier.url(&context.config), None)
                                    .map(|res| Node::Category(res))
                                    .wait()
                                    .map(|u| Some(u))
                },
                (&Service::Stores, &Model::Attribute) => {
                                context.request::<Attribute>(Method::Get, identifier.url(&context.config), None)
                                    .map(|res| Node::Attribute(res))
                                    .wait()
                                    .map(|u| Some(u))
                },
                (&Service::Stores, _) => {
                                Err(FieldError::new(
                                    "Could not get model from stores microservice.",
                                    graphql_value!({ "internal_error": "Unknown model" })
                                ))
                }
                (&Service::Orders, _) => {
                                Err(FieldError::new(
                                    "Could not get model from orders microservice.",
                                    graphql_value!({ "internal_error": "Unknown model" })
                                ))
                }
            }
        }
    }

    field languages(&executor) -> Vec<LanguageGraphQl> as "Fetches languages." {
        Language::as_vec()
    }


    field currencies(&executor) -> Vec<CurrencyGraphQl> as "Fetches currencies." {
        Currency::as_vec()
    }

    field categories(&executor) -> FieldResult<Option<Category>> as "Fetches categories tree." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::Category.to_url());

        context.request::<Category>(Method::Get, url, None)
            .wait()
            .map(|u| Some(u))
    }

    field attributes(&executor) -> FieldResult<Option<Vec<Attribute>>> as "Fetches all attributes." {
        let context = executor.context();
        let url = format!("{}/{}",
            context.config.service_url(Service::Stores),
            Model::Attribute.to_url());

        context.request::<Vec<Attribute>>(Method::Get, url, None)
            .wait()
            .map(|u| Some(u))
    }

    field search(&executor) -> Search as "Search endpoint" {
        Search{}
    }

    field main_page(&executor) -> MainPage as "Main page endpoint" {
        MainPage{}
    }

    field store(&executor, id: i32 as "Int id of a store.") -> FieldResult<Option<Store>> as "Fetches store by id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::Store.to_url(),
            id.to_string()
        );

        context.request::<Store>(Method::Get, url, None)
            .wait()
            .map(|u| Some(u))
    }

    field base_product(&executor, id: i32 as "Int Id of a base product.") -> FieldResult<Option<BaseProduct>> as "Fetches base product by id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            id.to_string()
        );

        context.request::<BaseProduct>(Method::Get, url, None)
            .wait()
            .map(|u| Some(u))
    }

});
