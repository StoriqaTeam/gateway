use chrono::prelude::*;
use futures::prelude::*;
use hyper;
use hyper::header::{Authorization, Cookie, Headers};
use juniper;
use juniper::FieldError;
use serde::de::DeserializeOwned;
use uuid::Uuid;

use config::Config;

use stq_api::rpc_client::RestApiClient;
use stq_http::client::{ClientHandle, Error};
use stq_http::request_util::{CorrelationToken, Currency as CurrencyHeader, RequestTimeout};
use stq_routes::service::Service;
use stq_static_resources::Currency;
use stq_types::SessionId;

use graphql::models::jwt::JWTPayload;

pub struct Context {
    pub http_client: ClientHandle,
    pub user: Option<JWTPayload>,
    pub session_id: Option<SessionId>,
    pub currency: Option<Currency>,
    pub uuid: String,
    pub config: Config,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(
        http_client: ClientHandle,
        user: Option<JWTPayload>,
        session_id: Option<SessionId>,
        currency: Option<Currency>,
        config: Config,
    ) -> Self {
        let uuid = Uuid::new_v4().hyphenated().to_string();
        Context {
            http_client,
            user,
            session_id,
            currency,
            uuid,
            config,
        }
    }

    pub fn get_rest_api_client(&self, s: Service) -> RestApiClient {
        RestApiClient::new(&self.config.service_url(s), self.user.clone().map(|u| u.user_id))
    }

    pub fn request<T>(&self, method: hyper::Method, url: String, body: Option<String>) -> Box<Future<Item = T, Error = FieldError> + Send>
    where
        T: DeserializeOwned + 'static + Send,
    {
        let mut headers = Headers::new();
        if let Some(ref user) = self.user {
            headers.set(Authorization(user.to_string()));
        };
        let mut cookie = Cookie::new();
        cookie.append("UUID", self.uuid.clone());
        if let Some(ref session_id) = self.session_id {
            cookie.append("SESSION_ID", session_id.to_string());
        };
        if let Some(ref currency) = self.currency {
            headers.set(CurrencyHeader(currency.code().into()));
        };
        headers.set(cookie);

        headers.set(CorrelationToken(self.uuid.clone()));
        headers.set(RequestTimeout(self.get_request_timeout().to_string()));

        let dt = Local::now();
        let correlation_token = self.uuid.clone();

        Box::new(
            self.http_client
                .request(method, url.clone(), body, Some(headers))
                .map_err(Error::into_graphql)
                .then(move |r| {
                    let d = Local::now() - dt;
                    match r {
                        Err(e) => {
                            info!(
                                "Request to microservice: {:?} failed with error `{:?}`, elapsed time: {}.{:03}, correlation token: {}",
                                url,
                                e,
                                d.num_seconds(),
                                d.num_milliseconds(),
                                correlation_token,
                            );
                            Err(e)
                        }
                        Ok(x) => {
                            info!(
                                "Request to microservice: {:?}, elapsed time: {}.{:03}, correlation token: {}",
                                url,
                                d.num_seconds(),
                                d.num_milliseconds(),
                                correlation_token,
                            );
                            Ok(x)
                        }
                    }
                }),
        )
    }

    fn get_request_timeout(&self) -> u64 {
        self.config.gateway.http_timeout_ms
    }
}
