//! Mod containing all graphql schema
pub mod attribute;
pub mod base_product;
pub mod cart;
pub mod cart_product;
pub mod cart_store;
pub mod category;
pub mod main_page;
pub mod moderator_comment;
pub mod mutations;
pub mod node;
pub mod order;
pub mod order_history;
pub mod page_info;
pub mod product;
pub mod query;
pub mod search;
pub mod store;
pub mod user;
pub mod user_delivery_address;
pub mod user_role;
pub mod warehouse;
pub mod warehouse_product;
pub mod wizard_store;

use juniper;

pub use self::mutations::*;
pub use self::node::*;
pub use self::query::*;

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
