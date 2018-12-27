use failure::Error;
use futures::Future;

mod saga;
pub use self::saga::*;

pub type ApiFuture<T> = Box<Future<Item = T, Error = Error>>;
