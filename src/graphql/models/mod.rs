pub mod model;
pub mod gender;
pub mod user;
pub mod product;
pub mod store;
pub mod connection;
pub mod id;
pub mod provider;
pub mod service;
pub mod jwt;


pub use self::model::Model;
pub use self::gender::Gender;
pub use self::user::{UpdateUser, User};
pub use self::product::{NewProduct, Product, UpdateProduct};
pub use self::store::{NewStore, Store, UpdateStore};
pub use self::connection::{Connection, Edge, PageInfo};
pub use self::id::ID;
pub use self::provider::Provider;
pub use self::service::Service;
pub use self::jwt::{ProviderOauth, JWT};

pub struct Viewer;

pub struct StaticNodeIds;

pub enum Node {
    User(User),
    Store(Store),
    Product(Product),
    Viewer(Viewer),
}
