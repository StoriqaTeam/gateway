use regex::Regex;

type ParamsConverter = Fn(Vec<&str>) -> Option<Route>;

/// Router class maps regex to type-safe list of routes, defined by `enum Route`
pub struct Router {
    regex_and_converters: Vec<(Regex, Box<ParamsConverter>)>,
}

/// List of all routes with params for the app
#[derive(Clone)]
pub enum Route {
    Root,
    Graphql,
}

impl Router {
    pub fn new() -> Self {
        Router {
            regex_and_converters: Vec::new(),
        }
    }

    /// Adds mapping between regex and route
    pub fn add_route(&mut self, regex_pattern: &str, route: Route) -> &Self {
        self.add_route_with_params(regex_pattern, move |_| Some(route.clone()));
        self
    }

    /// Adds mapping between regex and route with params
    /// converter is a function with argument being a set of regex matches (strings) for route params in regex
    /// this is needed if you want to convert params from strings to int or some other types
    pub fn add_route_with_params<F>(&mut self, regex_pattern: &str, converter: F) -> &Self
    where
        F: Fn(Vec<&str>) -> Option<Route> + 'static,
    {
        let regex = Regex::new(regex_pattern).unwrap();
        self.regex_and_converters.push((regex, Box::new(converter)));
        self
    }

    /// Tests string router for matches
    /// Returns Some(route) if there's a match
    pub fn test(&self, route: &str) -> Option<Route> {
        self.regex_and_converters
            .iter()
            .fold(None, |acc, ref regex_and_converter| {
                if acc.is_some() {
                    return acc;
                }
                Router::get_matches(&regex_and_converter.0, route).and_then(|params| regex_and_converter.1(params))
            })
    }

    fn get_matches<'a>(regex: &Regex, string: &'a str) -> Option<Vec<&'a str>> {
        regex.captures(string).and_then(|captures| {
            captures
                .iter()
                .skip(1)
                .fold(Some(Vec::<&str>::new()), |mut maybe_acc, maybe_match| {
                    if let Some(ref mut acc) = maybe_acc {
                        if let Some(mtch) = maybe_match {
                            acc.push(mtch.as_str());
                        }
                    }
                    maybe_acc
                })
        })
    }
}

pub fn create_router() -> Router {
    let mut router = Router::new();
    router.add_route(r"^/$", Route::Root);
    router.add_route(r"^/graphql$", Route::Graphql);
    router
}
