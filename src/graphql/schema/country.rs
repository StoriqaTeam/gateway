//! File containing Category object of graphql schema
use graphql::context::Context;
use graphql::models::*;

graphql_object!(Country: Context as "Country" |&self| {
    description: "Country info."

    field label() -> &str as "Label"{
        &self.label.0
    }

    field parent_label() -> Option<String> as "Parent label"{
        self.parent_label.clone().map(|p| p.0)
    }

    field level() -> &i32 as "Level" {
        &self.level
    }

    field children() -> &[Country] as "Children countries" {
        &self.children
    }

    field alpha2() -> &str as "alpha2" {
        &self.alpha2
    }

    field alpha3() -> &str as "alpha3" {
        &self.alpha3
    }

    field numeric() -> &i32 as "numeric" {
        &self.numeric
    }
});
