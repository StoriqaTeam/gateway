pub mod gender;
pub mod user;
pub mod product;
pub mod base_product;
pub mod search_product;
pub mod store;
pub mod connection;
pub mod id;
pub mod provider;
pub mod jwt;
pub mod user_role;
pub mod category;
pub mod attribute;

pub use self::gender::Gender;
pub use self::user::*;
pub use self::product::*;
pub use self::base_product::*;
pub use self::search_product::*;
pub use self::store::*;
pub use self::attribute::*;
pub use self::connection::{Connection, Edge, PageInfo};
pub use self::id::ID;
pub use self::provider::Provider;
pub use self::jwt::{CreateJWTEmailInput, CreateJWTProviderInput, ProviderOauth, UserStatus, JWT};
pub use self::user_role::{NewUserRole, Role, UserRole};
pub use self::category::*;

use graphql::context::Context;

pub struct Mock;

graphql_object!(Mock: Context as "Mock" |&self| {
    description: "Mock field."

    field mock() -> String as "Mock"{
        "Mock".to_string()
    }

});

pub struct Search;
