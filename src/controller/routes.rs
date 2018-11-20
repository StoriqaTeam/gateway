use stq_router::RouteParser;

/// List of all routes with params for the app
#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    Root,
    Graphql,
    Healthcheck,
    VerifyEmailApply(String),
    ResetPasswordApply(String),
    AddDeviceApply(String),
}

pub fn create_route_parser() -> RouteParser<Route> {
    let mut router = RouteParser::default();
    router.add_route(r"^/$", || Route::Root);
    router.add_route(r"^/graphql$", || Route::Graphql);
    router.add_route(r"^/healthcheck$", || Route::Healthcheck);
    router.add_route_with_params(r"^/verify-email-apply/(\S+)$", |params| {
        params.get(0).map(|s| s.to_string()).map(Route::VerifyEmailApply)
    });
    router.add_route_with_params(r"^/reset-password-apply/(\S+)$", |params| {
        params.get(0).map(|s| s.to_string()).map(Route::ResetPasswordApply)
    });
    router.add_route_with_params(r"^/add-device-apply/(\S+)$", |params| {
        params.get(0).map(|s| s.to_string()).map(Route::AddDeviceApply)
    });
    router
}
