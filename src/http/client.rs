use std::sync::mpsc;
use std::thread;
use std::result;

use tokio_core::reactor::{Core, Handle};
use hyper;
use futures::{future, Future};
use serde_json;

use super::utils;

pub type ClientResult = Result<String, Error>;
type Sender = mpsc::Sender<ClientResult>;

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

  pub fn send(&self, method: hyper::Method, url: String, body: Option<String>, callback: mpsc::Sender<ClientResult>) {
    let payload = Payload {
      url,
      method,
      body,
      callback
    };

    self.tx.send(payload);
  }

  pub fn send_sync(&self, method: hyper::Method, url: String, body: Option<String>) -> ClientResult {
    let (tx, rx) = mpsc::channel::<ClientResult>();
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
          .map_err(|err| Error::Network(err))
          .and_then(|res| {
            let status = res.status();
            let body_future: Box<future::Future<Item = String, Error = Error>> = Box::new(utils::read_body(res.body()).map_err(|err| Error::Network(err)));
            match status {
              hyper::StatusCode::Ok => 
                body_future,

              _ =>
                Box::new(
                  body_future.and_then(move |body| {
                    let message = serde_json::from_str::<ErrorMessage>(&body).ok();
                    let error = Error::Api(status, message);
                    future::err(error)
                  })
                )
              }
            })
          .map(|body| {
            callback.send(Ok(body));
          })
          .map_err(|err| {
            callback.send(Err(err));
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
  pub callback: mpsc::Sender<ClientResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    code: u16,
    message: String
}

#[derive(Debug)]
pub enum Error {
  Api(hyper::StatusCode, Option<ErrorMessage>),
  Network(hyper::Error),
  Unknown,
}
