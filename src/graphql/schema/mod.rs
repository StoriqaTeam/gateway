//! Mod containing all graphql schema
pub mod query;
pub mod mutations;
pub mod user;
pub mod node;
pub mod product;
pub mod store;

use juniper;

pub use self::query::*;
pub use self::mutations::*;
pub use self::node::*;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create() -> Schema {
    let query = Query {};
    let mutation = Mutation {};
    Schema::new(query, mutation)
}
