use stq_router::RouteParser;

/// List of all routes with params for the app
#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    Root,
    Graphql,
    Healthcheck,
    AppleAppSiteAssociation,
}

pub fn create_route_parser() -> RouteParser<Route> {
    let mut router = RouteParser::default();
    router.add_route(r"^/$", || Route::Root);
    router.add_route(r"^/graphql$", || Route::Graphql);
    router.add_route(r"^/healthcheck$", || Route::Healthcheck);
    router.add_route(r"^/apple-app-site-association$", || Route::AppleAppSiteAssociation);
    router
}
