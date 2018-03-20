//! Mod containing all graphql schema
pub mod query;
pub mod mutations;
pub mod user;
pub mod node;
pub mod product;
pub mod base_product;
pub mod store;
pub mod category;
pub mod attribute;
pub mod search;
pub mod user_role;
pub mod main_page;

use juniper;

pub use self::query::*;
pub use self::mutations::*;
pub use self::node::*;

use graphql::context::Context;
use graphql::models::Mock;


pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create() -> Schema {
    let query = Query {};
    let mutation = Mutation {};
    Schema::new(query, mutation)
}

graphql_object!(Mock: Context as "Mock" |&self| {
    description: "Mock field."

    field mock() -> String as "Mock"{
        "Mock".to_string()
    }

});