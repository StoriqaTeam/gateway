//! File containing query object of graphql schema
use std::str::FromStr;

use futures::Future;
use hyper::Method;
use juniper::ID as GraphqlID;
use juniper::{FieldError, FieldResult};
use serde_json;

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
                    context.request::<Option<User>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::User))
                },
                (&Service::Users, _) => {
                    Err(FieldError::new(
                        "Could not get model from users microservice.",
                        graphql_value!({ "internal_error": "Unknown model" })
                    ))
                },
                (&Service::Stores, &Model::Store) => {
                    context.request::<Option<Store>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::Store))
                },
                (&Service::Stores, &Model::Product) => {
                    context.request::<Option<Product>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::Product))
                },
                (&Service::Stores, &Model::BaseProduct) => {
                    context.request::<Option<BaseProduct>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::BaseProduct))
                },
                (&Service::Stores, &Model::Category) => {
                    context.request::<Option<Category>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::Category))
                },
                (&Service::Stores, &Model::Attribute) => {
                    context.request::<Option<Attribute>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::Attribute))
                },
                (&Service::Stores, _) => {
                                Err(FieldError::new(
                                    "Could not get model from stores microservice.",
                                    graphql_value!({ "internal_error": "Unknown model" })
                                ))
                },
                (&Service::Orders, &Model::Order) => {
                    context.request::<Option<Order>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::Order))
                },
                (&Service::Orders, _) => {
                    Err(FieldError::new(
                        "Could not get model from orders microservice.",
                        graphql_value!({ "internal_error": "Unknown model" })
                    ))
                },
                (&Service::Warehouses, &Model::Warehouse) => {
                    context.request::<Option<Warehouse>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::Warehouse))
                },
                (&Service::Warehouses, &Model::WarehouseProduct) => {
                    context.request::<Option<WarehouseProduct>>(Method::Get, identifier.url(&context.config), None)
                        .wait()
                        .map(|res| res.map(Node::WarehouseProduct))
                },
                (&Service::Warehouses, _) => {
                    Err(FieldError::new(
                        "Could not get model from warehouses microservice.",
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

        context.request::<Option<Category>>(Method::Get, url, None)
            .wait()
    }

    field currency_exchange(&executor) -> FieldResult<Option<CurrencyExchange>> as "Fetches currency exchange." {
        let context = executor.context();
        let url = format!("{}/currency_exchange",
            context.config.service_url(Service::Stores));

        context.request::<Option<CurrencyExchange>>(Method::Get, url, None)
            .wait()
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

        context.request::<Option<Store>>(Method::Get, url, None)
            .wait()
    }

    field base_product(&executor, id: i32 as "Int Id of a base product.") -> FieldResult<Option<BaseProduct>> as "Fetches base product by id." {
        let context = executor.context();

        let url = format!(
            "{}/{}/{}",
            &context.config.service_url(Service::Stores),
            Model::BaseProduct.to_url(),
            id.to_string()
        );

        context.request::<Option<BaseProduct>>(Method::Get, url, None)
            .wait()
    }

    field cart(&executor) -> FieldResult<Option<Cart>> as "Fetches cart products." {
        let context = executor.context();
        if let Some(session_id) = context.session_id {
            if let Some(_) = context.user {
                let body = serde_json::to_string(&CartMergePayload {user_from: session_id})?;
                let url = format!("{}/cart/merge",
                    &context.config.service_url(Service::Orders));

                context.request::<CartHash>(Method::Post, url, Some(body))
                    .map (|hash| hash.into_iter()
                        .map(|(product_id, info)| OrdersCartProduct {
                            product_id,
                            quantity: info.quantity,
                            store_id: info.store_id,
                            selected: info.selected,
                            comment: info.comment,
                    }).collect::<Vec<OrdersCartProduct>>())
                    .map(|u| Some(Cart::new(u)))
                    .wait()
            } else {
                let url = format!("{}/cart/products",
                    &context.config.service_url(Service::Orders));

                context.request::<CartHash>(Method::Get, url, None)
                    .map (|hash| hash.into_iter()
                        .map(|(product_id, info)| OrdersCartProduct {
                            product_id,
                            quantity: info.quantity,
                            store_id: info.store_id,
                            selected: info.selected,
                            comment: info.comment,
                    }).collect::<Vec<OrdersCartProduct>>())
                    .map(|u| Some(Cart::new(u)))
                    .wait()
            }
        } else {
            if let Some(_) = context.user {
                let url = format!("{}/cart/products",
                    &context.config.service_url(Service::Orders));

                context.request::<CartHash>(Method::Get, url, None)
                    .map (|hash| hash.into_iter()
                        .map(|(product_id, info)| OrdersCartProduct {
                            product_id,
                            quantity: info.quantity,
                            store_id: info.store_id,
                            selected: info.selected,
                            comment: info.comment,
                    }).collect::<Vec<OrdersCartProduct>>())
                    .map(|u| Some(Cart::new(u)))
                    .wait()
            }  else {
                Ok (None)
            }
        }
    }

});
