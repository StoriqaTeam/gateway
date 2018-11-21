use stq_router::RouteParser;

/// List of all routes with params for the app
#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    Root,
    Graphql,
    Healthcheck,
    VerifyEmail(String),
    ResetPassword,
    RegisterDevice,
}

pub fn create_route_parser() -> RouteParser<Route> {
    let mut router = RouteParser::default();
    router.add_route(r"^/$", || Route::Root);
    router.add_route(r"^/graphql$", || Route::Graphql);
    router.add_route(r"^/healthcheck$", || Route::Healthcheck);
    router.add_route_with_params(r"^/verify_email/(\S+)$", |params| {
        params.get(0).map(|s| s.to_string()).map(Route::VerifyEmail)
    });
    router.add_route(r"^/reset_password$", || Route::ResetPassword);
    router.add_route(r"^/register_device", || Route::RegisterDevice);
    router
}
