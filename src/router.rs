use regex::{Regex, Match};

struct Error {
    message: String
}

type ParamsConverter = fn(Vec<&str>) -> Option<Route>;

struct Router {
    regex_and_converters: Vec<(Regex, ParamsConverter)>,
}

enum Route {
    Root,
    Graphql,
}

impl Router {
    pub fn new() -> Self {
        Router { regex_and_converters: Vec::new() }
    }

    pub fn add_route(&mut self, regex_pattern: &str, route: Route) -> &Self {
        self.add_route_with_params(regex_pattern, |_| Some(route));
        // let regex = Regex::new(regex_pattern).unwrap();
        // self.regex_and_converters.push((regex, converter));
        self
    }


    pub fn add_route_with_params(&mut self, regex_pattern: &str, converter: ParamsConverter) -> &Self {
        let regex = Regex::new(regex_pattern).unwrap();
        self.regex_and_converters.push((regex, converter));
        self
    }

    pub fn find(&self, route: &str) -> Option<Route> {
        self.regex_and_converters.iter().fold(None, |acc, ref regex_and_converter| {
            Router::get_matches(&regex_and_converter.0, route)
                .and_then(|params| regex_and_converter.1(params))
        })
    }

    fn get_matches<'a>(regex: &Regex, string: &'a str) -> Option<Vec<&'a str>> {
        regex.captures(string)
            .and_then(|captures| {
                captures.iter().skip(1).fold(Some(Vec::<&str>::new()), |mut maybe_acc, maybe_match| {
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
