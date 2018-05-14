use std::fmt;
use std::str::FromStr;

#[derive(GraphQLEnum, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[graphql(name = "Status", description = "Current status")]
pub enum Status {
    Draft,
    Moderation,
    Decline,
    Published,
}

impl FromStr for Status {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "draft" => Ok(Status::Draft),
            "moderation" => Ok(Status::Moderation),
            "decline" => Ok(Status::Decline),
            "published" => Ok(Status::Published),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::Draft => write!(f, "draft"),
            Status::Moderation => write!(f, "moderation"),
            Status::Decline => write!(f, "decline"),
            Status::Published => write!(f, "published"),
        }
    }
}
