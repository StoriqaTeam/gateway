//! File containing query object of graphql schema
use std::str::FromStr;

use juniper::FieldResult;
use graphql::context::Context;
use graphql::models::*;
use hyper::Method;
use futures::Future;
use juniper::ID as GraphqlID;

use super::*;

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

    field id() -> GraphqlID as "Unique id"{
        QUERY_NODE_ID.to_string().into()
    }

    field apiVersion() -> &str as "Current api version." {
        "1.0"
    }

    field static_node_id() -> FieldResult<StaticNodeIds> as "Static node id dictionary." {
        Ok(StaticNodeIds{})
    }

    field me(&executor) -> FieldResult<Option<User>> as "Fetches viewer for users." {
        let context = executor.context();
        let url = format!("{}/{}/current",
            Service::Users.to_url(&context.config),
            Model::User.to_url());
        context.http_client.request_with_auth_header::<User>(Method::Get, url, None, context.user.clone())
                    .or_else(|err| Err(err.into_graphql()))
                    .wait()
                    .map(|u| Some(u))
    }

    field node(&executor, id: GraphqlID as "Id of a node.") -> FieldResult<Node> as "Fetches graphql interface node by id."  {
        let context = executor.context();
        if id.to_string() == QUERY_NODE_ID.to_string() {
             Ok(Node::Query(Query{}))
        } else {
            let identifier = ID::from_str(&*id)?;
            match (&identifier.service, &identifier.model) {
                (&Service::Users, _) => {
                                context.http_client.request_with_auth_header::<User>(Method::Get, identifier.url(&context.config), None, context.user.clone())
                                    .map(|res| Node::User(res))
                                    .or_else(|err| Err(err.into_graphql()))
                                    .wait()
                },
                (&Service::Stores, &Model::Store) => {
                                context.http_client.request_with_auth_header::<Store>(Method::Get, identifier.url(&context.config), None, context.user.clone())
                                    .map(|res| Node::Store(res))
                                    .or_else(|err| Err(err.into_graphql()))
                                    .wait()
                },
                (&Service::Stores, &Model::Product) => {
                                context.http_client.request_with_auth_header::<Product>(Method::Get, identifier.url(&context.config), None, context.user.clone())
                                    .map(|res| Node::Product(res))
                                    .or_else(|err| Err(err.into_graphql()))
                                    .wait()
                },
                (&Service::Stores, _) => {
                                context.http_client.request_with_auth_header::<Store>(Method::Get, identifier.url(&context.config), None, context.user.clone())
                                    .map(|res| Node::Store(res))
                                    .or_else(|err| Err(err.into_graphql()))
                                    .wait()
                }
            }
        }
    }


    field stores_find(&executor, name = None : Option<String> as "Store name") -> FieldResult<Vec<Store>> as "Fetches stores by name using relay connection." {
        let context = executor.context();

        let url = format!("{}/{}/search?name={}",
            Service::Stores.to_url(&context.config),
            Model::Store.to_url(),
            name.unwrap_or_default());

        context.http_client.request_with_auth_header::<Vec<Store>>(Method::Get, url, None, context.user.clone())
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

    field stores_auto_complete(&executor, name_part = None : Option<String> as "Store name") -> FieldResult<Vec<String>> as "Fetches stores full name by name part using relay connection." {
        let context = executor.context();

        let url = format!("{}/{}/auto_complete?name_part={}",
            Service::Stores.to_url(&context.config),
            Model::Store.to_url(),
            name_part.unwrap_or_default());

        context.http_client.request_with_auth_header::<Vec<String>>(Method::Get, url, None, context.user.clone())
            .or_else(|err| Err(err.into_graphql()))
            .wait()
    }

});
