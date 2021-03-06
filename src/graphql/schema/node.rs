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
    Store(Box<Store>),
    Product(Product),
    BaseProduct(BaseProduct),
    Category(Category),
    SearchCategory(SearchCategory),
    Attribute(Attribute),
    CustomAttribute(CustomAttribute),
    Cart,
    CartProduct(CartProduct),
    CartStore(CartStore),
    Warehouse(Box<GraphQLWarehouse>),
    Order(Box<GraphQLOrder>),
    Stock(GraphQLStock),
    Company(Company),
    Package(Packages),
    CompanyPackage(CompaniesPackages),
    Payout(Payout),
}

graphql_interface!(Node: Context as "Node" |&self| {
    description: "The Node interface contains a single field,
        id, which is a ID!. The node root field takes a single argument,
        a ID!, and returns a Node. These two work in concert to allow refetching."

    field id() -> GraphqlID {
        match *self {
            Node::Query(_)  => QUERY_NODE_ID.to_string().into(),
            Node::User(User { ref id, .. })  => ID::new(Service::Users, Model::User, id.0).to_string().into(),
            Node::Store(ref s)  => ID::new(Service::Stores, Model::Store, s.id.0).to_string().into(),
            Node::Product(Product { ref id, .. })  => ID::new(Service::Stores, Model::Product, id.0).to_string().into(),
            Node::BaseProduct(BaseProduct { ref id, .. })  => ID::new(Service::Stores, Model::BaseProduct, id.0).to_string().into(),
            Node::Category(Category { ref id, .. })  => ID::new(Service::Stores, Model::Category, id.0).to_string().into(),
            Node::SearchCategory(ref c)  => ID::new(Service::Stores, Model::SearchCategory, c.0.id.0).to_string().into(),
            Node::Attribute(Attribute { ref id, .. })  => ID::new(Service::Stores, Model::Attribute, id.0).to_string().into(),
            Node::CustomAttribute(CustomAttribute { ref id, .. })  => ID::new(Service::Stores, Model::CustomAttribute, id.0).to_string().into(),
            Node::CartProduct(CartProduct { ref id, .. })  => ID::new(Service::Orders, Model::CartProduct, id.0).to_string().into(),
            Node::CartStore(CartStore { ref id, .. })  => ID::new(Service::Orders, Model::CartStore, id.0).to_string().into(),
            Node::Cart => ID::new(Service::Orders, Model::Cart, 0).to_string().into(),
            Node::Warehouse(ref w)  => w.0.id.to_string().into(),
            Node::Order(ref o)  => o.0.id.to_string().into(),
            Node::Stock(ref s)  => format!("{}{}", s.0.warehouse_id, s.0.product_id).into(),
            Node::Company(Company { ref id, .. })  => ID::new(Service::Delivery, Model::Company, id.0).to_string().into(),
            Node::Package(Packages { ref id, .. })  => ID::new(Service::Delivery, Model::Package, id.0).to_string().into(),
            Node::CompanyPackage(CompaniesPackages { ref id, .. })  => ID::new(Service::Delivery, Model::CompanyPackage, id.0).to_string().into(),
            Node::Payout(Payout { ref id, .. }) => id.to_string().into(),
        }
    }

    instance_resolvers: |_| {
        &Query => match *self { Node::Query(ref h) => Some(h), _ => None },
        &User => match *self { Node::User(ref h) => Some(h), _ => None },
        &Store => match *self { Node::Store(ref h) => Some(&**h), _ => None },
        &Product => match *self { Node::Product(ref h) => Some(h), _ => None },
        &BaseProduct => match *self { Node::BaseProduct(ref h) => Some(h), _ => None },
        &Category => match *self { Node::Category(ref h) => Some(h), _ => None },
        &SearchCategory => match *self { Node::SearchCategory(ref h) => Some(h), _ => None },
        &Attribute => match *self { Node::Attribute(ref h) => Some(h), _ => None },
        &CustomAttribute => match *self { Node::CustomAttribute(ref h) => Some(h), _ => None },
        &CartProduct => match *self { Node::CartProduct(ref h) => Some(h), _ => None },
        &CartStore => match *self { Node::CartStore(ref h) => Some(h), _ => None },
        &GraphQLWarehouse => match *self { Node::Warehouse(ref h) => Some(&**h), _ => None },
        &GraphQLOrder => match *self { Node::Order(ref h) => Some(&**h), _ => None },
        &GraphQLStock => match *self { Node::Stock(ref h) => Some(h), _ => None },
        &Company => match *self { Node::Company(ref h) => Some(h), _ => None },
        &Packages => match *self { Node::Package(ref h) => Some(h), _ => None },
        &CompaniesPackages => match *self { Node::CompanyPackage(ref h) => Some(h), _ => None },
        &Cart => None::<&Cart>,
        &Payout => match *self { Node::Payout(ref h) => Some(h), _ => None },
    }
});

graphql_object!(StaticNodeIds: Context as "StaticNodeIds" |&self| {

    field query_id(&executor) -> i32 as "Static query id." {
        QUERY_NODE_ID
    }
});
