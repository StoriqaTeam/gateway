//! File containing node object of graphql schema
use juniper::ID as GraphqlID;

use stq_routes::model::Model;
use stq_routes::service::Service;

use super::*;
use graphql::context::Context;
use graphql::models::*;

pub struct StaticNodeIds;

pub enum Node {
    Query(Query),
    User(User),
    Store(Store),
    Product(Product),
    BaseProduct(BaseProduct),
    Category(Category),
    SearchCategory(SearchCategory),
    Attribute(Attribute),
    CartProduct(CartProduct),
    CartStore(CartStore),
    Warehouse(Warehouse),
    WarehouseProduct(WarehouseProduct),
    Order(Order),
}

graphql_interface!(Node: Context as "Node" |&self| {
    description: "The Node interface contains a single field, 
        id, which is a ID!. The node root field takes a single argument, 
        a ID!, and returns a Node. These two work in concert to allow refetching."

    field id() -> GraphqlID {
        match *self {
            Node::Query(_)  => QUERY_NODE_ID.to_string().into(),
            Node::User(User { ref id, .. })  => ID::new(Service::Users, Model::User, *id).to_string().into(),
            Node::Store(Store { ref id, .. })  => ID::new(Service::Stores, Model::Store, *id).to_string().into(),
            Node::Product(Product { ref id, .. })  => ID::new(Service::Stores, Model::Product, *id).to_string().into(),
            Node::BaseProduct(BaseProduct { ref id, .. })  => ID::new(Service::Stores, Model::BaseProduct, *id).to_string().into(),
            Node::Category(Category { ref id, .. })  => ID::new(Service::Stores, Model::Category, *id).to_string().into(),
            Node::SearchCategory(ref c)  => ID::new(Service::Stores, Model::SearchCategory, c.0.id).to_string().into(),
            Node::Attribute(Attribute { ref id, .. })  => ID::new(Service::Stores, Model::Attribute, *id).to_string().into(),
            Node::CartProduct(CartProduct { ref id, .. })  => ID::new(Service::Orders, Model::CartProduct, *id).to_string().into(),
            Node::CartStore(CartStore { ref id, .. })  => ID::new(Service::Orders, Model::CartStore, *id).to_string().into(),
            Node::Warehouse(Warehouse { ref id, .. })  => id.clone().into(),
            Node::WarehouseProduct(WarehouseProduct { ref id, .. })  => ID::new(Service::Warehouses, Model::WarehouseProduct, *id).to_string().into(),
            Node::Order(Order { ref id, .. })  => id.clone().into(),
        }
    }

    instance_resolvers: |_| {
        &Query => match *self { Node::Query(ref h) => Some(h), _ => None },
        &User => match *self { Node::User(ref h) => Some(h), _ => None },
        &Store => match *self { Node::Store(ref h) => Some(h), _ => None },
        &Product => match *self { Node::Product(ref h) => Some(h), _ => None },
        &BaseProduct => match *self { Node::BaseProduct(ref h) => Some(h), _ => None },
        &Category => match *self { Node::Category(ref h) => Some(h), _ => None },
        &SearchCategory => match *self { Node::SearchCategory(ref h) => Some(h), _ => None },
        &Attribute => match *self { Node::Attribute(ref h) => Some(h), _ => None },
        &CartProduct => match *self { Node::CartProduct(ref h) => Some(h), _ => None },
        &CartStore => match *self { Node::CartStore(ref h) => Some(h), _ => None },
        &Warehouse => match *self { Node::Warehouse(ref h) => Some(h), _ => None },
        &WarehouseProduct => match *self { Node::WarehouseProduct(ref h) => Some(h), _ => None },
        &Order => match *self { Node::Order(ref h) => Some(h), _ => None },
    }
});

graphql_object!(StaticNodeIds: Context as "StaticNodeIds" |&self| {

    field query_id(&executor) -> i32 as "Static query id." {
        QUERY_NODE_ID
    }
});
