use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq)]
pub struct FeeId(i32);

impl FeeId {
    pub fn new(id: i32) -> Self {
        FeeId(id)
    }

    pub fn inner(&self) -> &i32 {
        &self.0
    }
}

impl FromStr for FeeId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.parse()?;
        Ok(FeeId::new(id))
    }
}

impl Display for FeeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{}", self.0,))
    }
}
