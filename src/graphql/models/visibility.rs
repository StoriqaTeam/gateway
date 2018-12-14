use juniper::{FieldError, Value};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum Visibility {
    Active,
    Published,
}

impl Visibility {
    const ACTIVE_STR: &'static str = "active";
    const PUBLISHED_STR: &'static str = "published";
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Visibility::Active => Visibility::ACTIVE_STR,
            Visibility::Published => Visibility::PUBLISHED_STR,
        };
        write!(f, "{}", s)
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Published
    }
}

impl FromStr for Visibility {
    type Err = FieldError;

    fn from_str(visibility: &str) -> Result<Self, Self::Err> {
        match visibility.to_ascii_lowercase().as_ref() {
            Visibility::ACTIVE_STR => Ok(Visibility::Active),
            Visibility::PUBLISHED_STR => Ok(Visibility::Published),
            other => Err(FieldError::new(
                "Unknown visibility value",
                graphql_value!({
                    "code": 300,
                    "details": {
                        format!("Cannot resolve visibility. Unknown value: '{}'", other)
                    }
                }),
            )),
        }
    }
}

graphql_scalar!(Visibility {
    description: "Represents visibility of nodes. Possible values: ['active', 'published']"

    resolve(&self) -> Value {
        Value::string(&self.to_string())
    }

    from_input_value(v: &InputValue) -> Option<Visibility> {
        v.as_string_value().and_then(|s| Visibility::from_str(s).ok())
    }
});
