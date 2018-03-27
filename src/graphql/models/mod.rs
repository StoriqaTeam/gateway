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
pub mod reset_password;
pub mod email_verify;

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
pub use self::user_role::{NewUserRoleInput, Role, UserRoles};
pub use self::category::*;
pub use self::reset_password::*;
pub use self::email_verify::*;

//Mock object, made to return from graphql when microservices responce contains nothing - '()'
pub struct Mock;

//Search object, made to add endpoints for searching for unauthorized users
pub struct Search;

//MainPage object, made for displaying info at main page
pub struct MainPage;
