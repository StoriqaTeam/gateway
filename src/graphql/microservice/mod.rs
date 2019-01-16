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

pub type ApiFuture<T> = Box<Future<Item = T, Error = Error>>;
