pub mod attribute;
pub mod base_product;
pub mod cart;
pub mod category;
pub mod connection;
pub mod email_verify;
pub mod gender;
pub mod id;
pub mod jwt;
pub mod product;
pub mod provider;
pub mod reset_password;
pub mod search_product;
pub mod store;
pub mod user;
pub mod user_role;

pub use self::attribute::*;
pub use self::base_product::*;
pub use self::cart::*;
pub use self::category::*;
pub use self::connection::*;
pub use self::email_verify::*;
pub use self::gender::Gender;
pub use self::id::ID;
pub use self::jwt::{CreateJWTEmailInput, CreateJWTProviderInput, ProviderOauth, UserStatus, JWT};
pub use self::product::*;
pub use self::provider::Provider;
pub use self::reset_password::*;
pub use self::search_product::*;
pub use self::store::*;
pub use self::user::*;
pub use self::user_role::{NewUserRoleInput, Role, UserRoles};

//Mock object, made to return from graphql when microservices responce contains nothing - '()'
pub struct Mock;

//Search object, made to add endpoints for searching for unauthorized users
pub struct Search;

//MainPage object, made for displaying info at main page
pub struct MainPage;
