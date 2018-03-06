use std::fmt;

#[derive(GraphQLEnum, Debug, Clone, Serialize)]
#[graphql(name = "Provider", description = "Token providers")]
pub enum Provider {
    #[graphql(description = "Google")]
    Google,

    #[graphql(description = "Facebook")]
    Facebook,

    #[graphql(description = "Email")]
    Email,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Provider::Facebook => write!(f, "facebook"),
            Provider::Google => write!(f, "google"),
            Provider::Email => write!(f, "email"),
        }
    }
}
