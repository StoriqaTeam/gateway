use failure::Error;
use futures::Future;

mod saga;
mod stores;
pub use self::saga::*;
pub use self::stores::*;

mod billing;
pub use self::billing::*;

mod delivery;
pub use self::delivery::*;

mod order;
pub use self::order::*;

pub mod requests;
pub use self::requests::*;

pub mod responses;
pub use self::responses::*;

pub type ApiFuture<T> = Box<Future<Item = T, Error = Error>>;
