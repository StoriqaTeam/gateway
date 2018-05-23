pub mod address;
pub mod attribute;
pub mod base_product;
pub mod cart;
pub mod category;
pub mod connection;
pub mod currency_exchange;
pub mod email_verify;
pub mod gender;
pub mod id;
pub mod jwt;
pub mod moderator_comment;
pub mod product;
pub mod provider;
pub mod reset_password;
pub mod search_product;
pub mod status;
pub mod store;
pub mod user;
pub mod user_delivery_address;
pub mod user_role;
pub mod wizard_store;
pub mod warehouse;

pub use self::address::*;
pub use self::attribute::*;
pub use self::base_product::*;
pub use self::cart::*;
pub use self::category::*;
pub use self::connection::*;
pub use self::currency_exchange::*;
pub use self::email_verify::*;
pub use self::gender::Gender;
pub use self::id::ID;
pub use self::jwt::{CreateJWTEmailInput, CreateJWTProviderInput, ProviderOauth, UserStatus, JWT};
pub use self::moderator_comment::*;
pub use self::product::*;
pub use self::provider::Provider;
pub use self::reset_password::*;
pub use self::search_product::*;
pub use self::status::*;
pub use self::store::*;
pub use self::user::*;
pub use self::user_delivery_address::*;
pub use self::user_role::*;
pub use self::warehouse::*;
pub use self::wizard_store::*;

//Mock object, made to return from graphql when microservices responce contains nothing - '()'
pub struct Mock;

//Search object, made to add endpoints for searching for unauthorized users
pub struct Search;

//MainPage object, made for displaying info at main page
pub struct MainPage;
