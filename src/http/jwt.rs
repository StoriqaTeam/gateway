use std::fmt;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JWTPayload {
    pub user_id: i32,
}

impl Display for JWTPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.user_id.fmt(f)
    }
}
