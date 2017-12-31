use std::sync::Arc;

use tokio_core::reactor::{Handle};
use hyper;
use futures::future::IntoFuture;
use futures::{future, Future};
use futures::sync::{mpsc, oneshot};
use futures::stream::{Stream};
use futures::sink::Sink;
use serde_json;
use juniper;

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
        return Box::new(callback.send(Err(Error::Unknown)).into_future().map(|_| ()).map_err(|_| ()))
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

impl ClientHandle {
  pub fn send(&self, method: hyper::Method, url: String, body: Option<String>) -> Box<Future<Item=String, Error=Error>> {
    info!("Starting outbound http request: {} {} with body {}", method, url, body.clone().unwrap_or_default());

    let (tx, rx) = oneshot::channel::<ClientResult>();
    let payload = Payload {
      url,
      method,
      body,
      callback: tx,
    };


    let future = self.tx.send(payload)
      .map_err(|err| {
        error!("Unexpected error sending http client request params to channel: {}", err);
        Error::Unknown
      })
      .and_then(|_| {
        rx.map_err(|err| {
          error!("Unexpected error receiving http client response from channel: {}", err);
          Error::Unknown
        })
      })
      .and_then(|result| result);

    Box::new(future)
  }
}


struct Payload {
  pub url: String,
  pub method: hyper::Method,
  pub body: Option<String>,
  pub callback: oneshot::Sender<ClientResult>,
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

impl Error {
  pub fn to_graphql(&self, service_name: &str) -> juniper::FieldError {
    let description = format!("Error response from {} microservice", service_name);
    match *self {
      Error::Api(status, Some(ErrorMessage { code, message })) => {
        let code = code.to_string();
        let status = status.to_string();
        juniper::FieldError::new(
            description,
            graphql_value!({ "status": status, "code": code, "message": message }),
        )
      },
      Error::Api(status, None) => {
        let status = status.to_string();
        juniper::FieldError::new(
            description,
            graphql_value!({ "status": status }),
        )
      },
      Error::Network(_) => {
        juniper::FieldError::new(
            description,
            graphql_value!("See logs for details."),
        )
      }
      _ => {
          juniper::FieldError::new(
            description,
            graphql_value!("See logs for details."),
          )
      }
    }
  }
}
