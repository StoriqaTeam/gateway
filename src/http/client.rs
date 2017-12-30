use std::sync::Arc;
use std::thread;

use tokio_core::reactor::{Core, Handle};
use hyper;
use futures::{future, Future};
use futures::sync::mpsc;
use futures::stream::{Stream};
use futures::sink::Sink;
use serde_json;

use super::utils;
use ::config::Config;

pub type ClientResult = Result<String, Error>;

pub struct Client {
  client: hyper::Client<hyper::client::HttpConnector>,
  tx: mpsc::Sender<Payload>,
  rx: mpsc::Receiver<Payload>,
}

impl Client {
  pub fn new(config: &Config, handle: &Handle) -> Self {
    let (tx, rx) = mpsc::channel::<Payload>(config.gateway.http_client_buffer_size);
    let client = hyper::Client::new(handle);
    Client { client, tx, rx }
  }

  pub fn stream(&self) -> Box<Stream<Item=(), Error=()>> {
    let client = self.client.clone();
    Box::new(
      self.rx.and_then(move |payload| {
        Self::send_request(client, payload).map(|_| ()).map_err(|_| ())
      })
    )
  }

  pub fn handle(&self) -> ClientHandle {
    ClientHandle {
      tx: self.tx.clone()
    }
  }

  fn send_request(client: hyper::Client<hyper::client::HttpConnector>, payload: Payload) -> Box<Future<Item=(), Error=()>> {
    let Payload { url, method, body: maybe_body, callback } = payload;

    let uri = match url.parse() {
      Ok(val) => val,
      Err(err) => {
        error!("Url `{}` passed to http client cannot be parsed: `{}`", url, err);
        return Box::new(callback.send(Err(Error::Unknown)).map(|_| ()).map_err(|_| ()))
      }
    };
    let mut req = hyper::Request::new(method, uri);
    for body in maybe_body.iter() {
      req.set_body(body.clone());
    }

    let url = Arc::new(url);
    let url_clone = url.clone();

    let task = client.request(req)
      .map_err(move |err| {
        error!("Error sending request to `{}`: {}", url, err);
        Error::Network(err)
      })
      .and_then(move |res| {
        let status = res.status();
        let body_future: Box<future::Future<Item = String, Error = Error>> = 
          Box::new(utils::read_body(res.body())
            .map_err(move |err| {
              error!("Error reading body from `{}`: {}", url_clone, err);
              Error::Network(err)
            })
          );
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
        .then(|result| callback.send(result))
        .map(|_| ()).map_err(|_| ());

    Box::new(task)
  }

}

#[derive(Clone)]
pub struct ClientHandle {
  tx: mpsc::Sender<Payload>,
}

// impl ClientHandle {
//   pub fn send_sync(&self, method: hyper::Method, url: String, body: Option<String>) -> ClientResult {
//     info!("Starting outbound http request: {} {} with body {}", method, url, body.clone().unwrap_or_default());

//     let (tx, rx) = mpsc::channel::<ClientResult>();
//     let payload = Payload {
//       url,
//       method,
//       body,
//       callback: tx,
//     };

//     if let Err(err) = self.tx.send(payload) {
//       error!("Unexpected error sending http client request params to actor: {}", err);
//       return Err(Error::Unknown)
//     };

//     match rx.recv() {
//       Ok(result) => result,
//       Err(err) => {
//         error!("Unexpected error sending http client request params to actor: {}", err);
//         Err(Error::Unknown)      
//       }
//     }
//   }


//   fn start_event_loop(rx: mpsc::Receiver<Payload>) {
//       let mut core = Core::new().expect("Unexpected error creating main event loop for http client");
//       let handle = core.handle();

//       let client = hyper::Client::new(&handle);

//       for payload in rx {
//         let task = Self::make_request(&client, payload);
//         if let Err(err) = core.run(task) {
//           error!("Unexpected error running http client on event loop: {:?}", err)
//         }
//       }
//   }

//   fn start(&self, rx: mpsc::Receiver<Payload>) {
//     thread::spawn(|| {
//       Self::start_event_loop(rx)
//     });
//   }

// }

struct Payload {
  pub url: String,
  pub method: hyper::Method,
  pub body: Option<String>,
  pub callback: mpsc::Sender<ClientResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: String
}

#[derive(Debug)]
pub enum Error {
  Api(hyper::StatusCode, Option<ErrorMessage>),
  Network(hyper::Error),
  Unknown,
}
