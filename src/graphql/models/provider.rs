use std::fmt;

#[derive(GraphQLEnum)]
#[graphql(name = "Provider", description = "Token providers")]
pub enum Provider {
    #[graphql(description = "Google")] 
    Google,

    #[graphql(description = "Facebook")] 
    Facebook,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Provider::Facebook => write!(f, "facebook"),
            Provider::Google => write!(f, "google"),
        }
    }
}