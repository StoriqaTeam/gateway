//! File containing node object of graphql schema
use juniper::ID as GraphqlID;
use juniper::FieldResult;
use stq_routes::model::Model;
use stq_routes::service::Service;

use graphql::context::Context;
use graphql::models::*;
use super::*;

pub struct StaticNodeIds;

pub enum Node {
    User(User),
    Store(Store),
    Product(Product),
    Query(Query),
}

graphql_interface!(Node: Context as "Node" |&self| {
    description: "The Node interface contains a single field, 
        id, which is a ID!. The node root field takes a single argument, 
        a ID!, and returns a Node. These two work in concert to allow refetching."
    
    field id() -> GraphqlID {
        match *self {
            Node::User(User { ref id, .. })  => ID::new(Service::Users, Model::User, *id).to_string().into(),
            Node::Store(Store { ref id, .. })  => ID::new(Service::Stores, Model::Store, *id).to_string().into(),
            Node::Product(Product { ref id, .. })  => ID::new(Service::Stores, Model::Product, *id).to_string().into(),
            Node::Query(_)  => QUERY_NODE_ID.to_string().into(),
        }
    }

    instance_resolvers: |_| {
        &User => match *self { Node::User(ref h) => Some(h), _ => None },
        &Store => match *self { Node::Store(ref h) => Some(h), _ => None },
        &Product => match *self { Node::Product(ref h) => Some(h), _ => None },
        &Query => match *self { Node::Query(ref h) => Some(h), _ => None },
    }
});

graphql_object!(StaticNodeIds: Context as "StaticNodeIds" |&self| {

    field query_id(&executor) -> FieldResult<i32> as "Static query id." {
        Ok(QUERY_NODE_ID)
    }
});
