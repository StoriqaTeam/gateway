use std::sync::Arc;
use std::time::Duration;

use chrono::prelude::*;
use failure::Error as FailureError;
use failure::Fail;
use futures::future;
use futures::prelude::*;
use futures_cpupool::CpuPool;
use hyper::header::{Authorization, Bearer};
use hyper::server::Request;
use hyper::Method::{Get, Post};
use jsonwebtoken::{decode, Algorithm, Validation};
use juniper::http::GraphQLRequest;
use serde_json;

use stq_http::client::{ClientHandle, HttpClient, TimeLimitedHttpClient};
use stq_http::controller::Controller;
use stq_http::controller::ControllerFuture;
use stq_http::errors::ErrorMessageWrapper;
use stq_http::request_util::parse_body;
use stq_http::request_util::serialize_future;
use stq_http::request_util::CorrelationToken;
use stq_http::request_util::Currency as CurrencyHeader;
use stq_http::request_util::SessionId as SessionIdHeader;
use stq_router::RouteParser;
use stq_static_resources::Currency;
use stq_types::SessionId;

use self::routes::Route;
use config::Config;
use errors::Error;
use graphql::context::Context;
use graphql::models::jwt::JWTPayload;
use graphql::schema::Schema;
use sentry_integration::log_and_capture_error;

pub mod graphiql;
pub mod routes;

pub struct ControllerImpl {
    jwt_public_key: Vec<u8>,
    route_parser: Arc<RouteParser<Route>>,
    cpu_pool: CpuPool,
    http_client: ClientHandle,
    jwt_leeway: i64,
    config: Config,
    schema: Arc<Schema>,
}

impl ControllerImpl {
    /// Create a new controller based on services
    pub fn new(
        http_client: ClientHandle,
        jwt_public_key: Vec<u8>,
        cpu_pool: CpuPool,
        jwt_leeway: i64,
        config: Config,
        schema: Arc<Schema>,
    ) -> Self {
        let route_parser = Arc::new(routes::create_route_parser());

        Self {
            jwt_leeway,
            http_client,
            jwt_public_key,
            route_parser,
            cpu_pool,
            config,
            schema,
        }
    }
}

impl Controller for ControllerImpl {
    fn call(&self, req: Request) -> ControllerFuture {
        let method = format!("{}", req.method());
        let path = req.path().to_string();
        let dt = Local::now();
        let config = self.config.clone();
        let leeway = self.jwt_leeway;
        let jwt_public_key = self.jwt_public_key.clone();
        let cpu_pool = self.cpu_pool.clone();
        let schema = self.schema.clone();

        let request_timeout = Duration::from_millis(self.config.gateway.http_timeout_ms);
        let client = TimeLimitedHttpClient::new(self.http_client.clone(), request_timeout);
        let saga_addr = self.config.saga_microservice.url.clone();

        Box::new(
            match (&req.method().clone(), self.route_parser.test(req.path())) {
                (&Get, Some(Route::Root)) => Box::new(future::ok(graphiql::source("/graphql"))),

                (&Post, Some(Route::Graphql)) => {
                    let headers = req.headers().clone();
                    let auth_header = headers.get::<Authorization<Bearer>>();

                    let mut validation = Validation {
                        leeway,
                        ..Validation::new(Algorithm::RS256)
                    };
                    let token_payload = auth_header.and_then(move |auth| {
                        let token = auth.0.token.as_ref();
                        decode::<JWTPayload>(token, jwt_public_key.as_ref(), &validation)
                            .ok()
                            .map(|t| t.claims)
                    });

                    let session_id_header = headers.get::<SessionIdHeader>().and_then(|sid| sid.parse::<SessionId>().ok());
                    let currency_header = headers.get::<CurrencyHeader>().and_then(|sid| sid.parse::<Currency>().ok());
                    let correlation_token = headers.get::<CorrelationToken>().map(|token| token.clone());

                    serialize_future::<_, FailureError, _>(
                        parse_body::<GraphQLRequest>(req.body())
                            .map_err(|e| {
                                e.context("Parsing body // POST /graphql in GraphQLRequest failed!")
                                    .context(Error::Parse)
                                    .into()
                            }).and_then(move |graphql_req| {
                                cpu_pool
                                    .spawn_fn(move || {
                                        let graphql_context = Context::new(
                                            client,
                                            token_payload,
                                            session_id_header,
                                            currency_header,
                                            config,
                                            correlation_token,
                                        );
                                        let resp = graphql_req.execute(&*schema, &graphql_context);
                                        serde_json::to_value(resp)
                                    }).map_err(From::from)
                            }),
                    )
                }

                (&Get, Some(Route::VerifyEmailApply(token))) => {
                    let body = json!({ "token": token }).to_string();
                    let url = format!("{}/email_verify_apply", saga_addr);
                    Box::new(
                        client
                            .request_json::<String>(Post, url.clone(), Some(body), None)
                            .map_err(From::from)
                            .then(move |r| {
                                match r {
                                    Err(e) => Err(e),
                                    Ok(_) => {
                                        Ok(r##"
                                            <!DOCTYPE html>
                                            <html lang="en">
                                            <head>
                                                <meta charset="UTF-8" />
                                                <meta http-equiv="X-UA-Compatible" content="ie=edge" />
                                                <title>Storiqa: email verification</title>
                                                <style>
                                                html {
                                                    height: 100%;
                                                    margin: 0px;
                                                }
                                                body {
                                                    background-color: #fafafa;
                                                    color: #03a9ff;
                                                    width: 100%;
                                                    height: 100%;
                                                    margin: 0px;
                                                }
                                                .wrapper {
                                                    height: 100%;
                                                    width: 100%;
                                                    display: flex;
                                                    flex: 1;
                                                    justify-content: center;
                                                    align-items: center;
                                                    font-family: Arial, Helvetica, sans-serif;
                                                    position: relative;
                                                }
                                                span {
                                                    font-size: 100px;
                                                    text-align: center;
                                                }
                                                img {
                                                    height: 25px;
                                                    width: 192px;
                                                    left: 20px;
                                                    top: 20px;
                                                    position: absolute;
                                                }
                                                </style>
                                            </head>
                                            <body>
                                                <img
                                                src="https://s3.eu-central-1.amazonaws.com/dumpster.stq/img/storiqa-logo.png"
                                                alt="logo"
                                                />
                                                <div class="wrapper"><span>Successfully verified email</span></div>
                                            </body>
                                            </html>
                                            "##.to_string())
                                    }
                                }
                            }),
                    )
                }

                (&Get, Some(Route::ResetPasswordApply(_))) => Box::new(future::ok(
                    r##"
                        <!DOCTYPE html>
                        <html lang="en">
                        <head>
                            <meta charset="UTF-8" />
                            <meta http-equiv="X-UA-Compatible" content="ie=edge" />
                            <title>Storiqa: reset password</title>
                            <style>
                            html {
                                height: 100%;
                                margin: 0px;
                            }
                            body {
                                background-color: #fafafa;
                                color: #03a9ff;
                                width: 100%;
                                height: 100%;
                                margin: 0px;
                            }
                            .wrapper {
                                height: 100%;
                                width: 100%;
                                display: flex;
                                flex: 1;
                                justify-content: center;
                                align-items: center;
                                font-family: Arial, Helvetica, sans-serif;
                                position: relative;
                            }
                            span {
                                font-size: 100px;
                                text-align: center;
                            }
                            img {
                                height: 25px;
                                width: 192px;
                                left: 20px;
                                top: 20px;
                                position: absolute;
                            }
                            </style>
                        </head>
                        <body>
                            <img
                            src="https://s3.eu-central-1.amazonaws.com/dumpster.stq/img/storiqa-logo.png"
                            alt="logo"
                            />
                            <div class="wrapper"><span>Please open this link on device.</span></div>
                        </body>
                        </html>
                        "##.to_string(),
                )),

                (&Get, Some(Route::AddDeviceApply(_))) => Box::new(future::ok(
                    r##"
                        <!DOCTYPE html>
                        <html lang="en">
                        <head>
                            <meta charset="UTF-8" />
                            <meta http-equiv="X-UA-Compatible" content="ie=edge" />
                            <title>Storiqa: add device</title>
                            <style>
                            html {
                                height: 100%;
                                margin: 0px;
                            }
                            body {
                                background-color: #fafafa;
                                color: #03a9ff;
                                width: 100%;
                                height: 100%;
                                margin: 0px;
                            }
                            .wrapper {
                                height: 100%;
                                width: 100%;
                                display: flex;
                                flex: 1;
                                justify-content: center;
                                align-items: center;
                                font-family: Arial, Helvetica, sans-serif;
                                position: relative;
                            }
                            span {
                                font-size: 100px;
                                text-align: center;
                            }
                            img {
                                height: 25px;
                                width: 192px;
                                left: 20px;
                                top: 20px;
                                position: absolute;
                            }
                            </style>
                        </head>
                        <body>
                            <img
                            src="https://s3.eu-central-1.amazonaws.com/dumpster.stq/img/storiqa-logo.png"
                            alt="logo"
                            />
                            <div class="wrapper"><span>Please open this link on device.</span></div>
                        </body>
                        </html>
                        "##.to_string(),
                )),

                // Fallback
                (m, _) => Box::new(future::err(
                    format_err!("Request to non existing endpoint in gateway microservice! {:?} {:?}", m, path)
                        .context(Error::NotFound)
                        .into(),
                )),
            }.then(move |res| {
                let d = Local::now() - dt;
                let message = match res {
                    Ok(_) => format!(
                        "Response with success: {} {}, elapsed time = {}.{:03}",
                        method,
                        path,
                        d.num_seconds(),
                        d.num_milliseconds()
                    ),
                    Err(ref e) => {
                        let wrapper = ErrorMessageWrapper::<Error>::from(&e);
                        if wrapper.inner.code == 500 {
                            log_and_capture_error(&e);
                        }

                        format!(
                            "Response with error {}: {} {}, elapsed time = {}.{:03}",
                            e,
                            method,
                            path,
                            d.num_seconds(),
                            d.num_milliseconds()
                        )
                    }
                };
                debug!("{}", message);
                res
            }),
        )
    }
}
