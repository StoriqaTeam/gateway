use std::sync::mpsc;
use std::thread;

use tokio_core::reactor::{Core, Handle};
use hyper;
use futures::Future;

use super::utils;

#[derive(Clone)]
pub struct Client {
  tx: mpsc::Sender<Payload>,
}

impl Client {
  pub fn new() -> Self {
    let (tx, rx) = mpsc::channel::<Payload>();
    let client_actor = 
      Client {
        tx,
      };
    client_actor.start(rx);
    client_actor
  }

  pub fn send(&self, method: hyper::Method, url: String, body: Option<String>, callback: mpsc::Sender<Result<String, hyper::Error>>) {
    let payload = Payload {
      url,
      method,
      body,
      callback
    };

    self.tx.send(payload);
  }

  pub fn send_sync(&self, method: hyper::Method, url: String, body: Option<String>) -> Result<String, hyper::Error> {
    let (tx, rx) = mpsc::channel();
    let payload = Payload {
      url,
      method,
      body,
      callback: tx,
    };

    self.tx.send(payload);
    rx.recv().unwrap()
  }


  fn start(&self, rx: mpsc::Receiver<Payload>) {
    thread::spawn(|| {
      let mut core = Core::new().expect("Unexpected error creating main event loop");
      let handle = core.handle();

      let client = hyper::Client::new(&handle);

      for payload in rx {
        let Payload { url, method, body: maybe_body, callback } = payload;

        let uri = url.parse().unwrap();
        let mut req = hyper::Request::new(method, uri);
        for body in maybe_body.iter() {
          req.set_body(body.clone());
        }

        let task = client.request(req)
          .and_then(|res| utils::read_body(res.body()))
          .map(|res| {
            callback.send(Ok(res));
          });

        core.run(task);
      }
    });    
  }

}

struct Payload {
  pub url: String,
  pub method: hyper::Method,
  pub body: Option<String>,
  pub callback: mpsc::Sender<Result<String, hyper::Error>>,
}


