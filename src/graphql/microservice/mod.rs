use failure::Error;
use futures::Future;

mod saga;
pub use self::saga::*;

mod billing;
pub use self::billing::*;

mod delivery;
pub use self::delivery::*;

pub type ApiFuture<T> = Box<Future<Item = T, Error = Error>>;
