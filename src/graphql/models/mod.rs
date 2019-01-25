pub mod address;
pub mod attribute;
pub mod base_product;
pub mod billing_info;
pub mod buy_now;
pub mod cart;
pub mod category;
pub mod company;
pub mod company_package;
pub mod connection;
pub mod country;
pub mod coupon;
pub mod currency_exchange;
pub mod custom_attribute;
pub mod delivery;
pub mod email_template;
pub mod email_verify;
pub mod fee;
pub mod id;
pub mod invoice;
pub mod jwt;
pub mod moderator_comment;
pub mod order;
pub mod order_billing;
pub mod package;
pub mod product;
pub mod reset_password;
pub mod reset_token;
pub mod search_product;
pub mod stock;
pub mod store;
pub mod stripe;
pub mod user;
pub mod user_delivery_address;
pub mod user_role;
pub mod visibility;
pub mod warehouse;
pub mod wizard_store;

pub use self::address::*;
pub use self::attribute::*;
pub use self::base_product::*;
pub use self::billing_info::*;
pub use self::buy_now::*;
pub use self::cart::*;
pub use self::category::*;
pub use self::company::*;
pub use self::company_package::*;
pub use self::connection::*;
pub use self::country::*;
pub use self::coupon::*;
pub use self::currency_exchange::*;
pub use self::custom_attribute::*;
pub use self::delivery::*;
pub use self::email_template::*;
pub use self::email_verify::*;
pub use self::fee::*;
pub use self::id::ID;
pub use self::invoice::*;
pub use self::jwt::{CreateJWTEmailInput, CreateJWTProviderInput, ProviderOauth, UserStatus, JWT};
pub use self::moderator_comment::*;
pub use self::order::*;
pub use self::order_billing::*;
pub use self::package::*;
pub use self::product::*;
pub use self::reset_password::*;
pub use self::reset_token::*;
pub use self::search_product::*;
pub use self::stock::*;
pub use self::store::*;
pub use self::stripe::*;
pub use self::user::*;
pub use self::user_delivery_address::*;
pub use self::user_role::*;
pub use self::visibility::*;
pub use self::warehouse::*;
pub use self::wizard_store::*;

//Mock object, made to return from graphql when microservices response contains nothing - '()'
pub struct Mock;

//Search object, made to add endpoints for searching for unauthorized users
pub struct Search;

//MainPage object, made for displaying info at main page
pub struct MainPage;

//Admin object, made for displaying info at admin page
pub struct Admin;

//FinancialManager object, made for displaying info at financial manager page
pub struct FinancialManager;
