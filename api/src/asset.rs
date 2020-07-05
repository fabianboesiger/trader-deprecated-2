use std::fmt;
use std::hash::Hash;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Asset(String);

impl fmt::Display for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Asset {
    fn from(string: String) -> Asset {
        Asset(string)
    }
}