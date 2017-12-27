use std::sync::mpsc;

use hyper;
use tokio_core::reactor::Handle;

pub struct ClientActor {
  client: hyper::Client<hyper::client::HttpConnector>,
  tx: mpsc::Sender<Payload>,
  rx: mpsc::Receiver<Payload>,
}

impl ClientActor {
  pub fn new(handle: &Handle) -> Self {
    let (tx, rx) = mpsc::channel();
    let client = hyper::Client::new(handle);
    ClientActor {
      client,
      tx,
      rx
    }
  }

  // pub fn remote(&self) -> Client {
  //   Client {
  //     tx: self.tx,
  //   }
  // }
}

pub struct Client {
  tx: mpsc::Sender<Payload>,
}

impl Client {
  pub fn send(&self, url: String, method: hyper::Method, body: Option<String>, callback: mpsc::Sender<Result<String, hyper::Error>>) {
    let payload = Payload {
      url,
      method,
      body,
      callback
    };

    self.tx.send(payload);
  }

  pub fn send_sync(&self, url: String, method: hyper::Method, body: Option<String>) -> Result<String, hyper::Error> {
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
}

struct Payload {
  pub url: String,
  pub method: hyper::Method,
  pub body: Option<String>,
  pub callback: mpsc::Sender<Result<String, hyper::Error>>,
}


