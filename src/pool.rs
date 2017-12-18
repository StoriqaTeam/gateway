use hyper::client::{Client, HttpConnector};
// use std::io::{self, Write};
// use futures::{Future, Stream};
use tokio_core::reactor::Handle;

#[derive(Clone)]
pub struct Pool {
    connection: Client<HttpConnector>,
    base_url: String,
}

impl Pool {
    pub fn new(base_url: String, handle: &Handle) -> Self {
        // by default keep_alive is true
        let client = Client::configure().no_proto().build(handle);

        Pool {
            connection: client,
            base_url: base_url,
        }
    }

    // pub fn get() -> Box<Future<Item = Self::Response, Error = Self::Error>>
}


#[cfg(test)]
mod tests {
    use pool::Pool;

    #[test]
    fn can_create_pool() {
        let pool = Pool::new("http://0.0.0.0:8000/").unwrap();
    }

}
