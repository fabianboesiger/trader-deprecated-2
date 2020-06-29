use std::hash::{Hash, Hasher};
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Asset(String);

impl From<String> for Asset {
    fn from(string: String) -> Asset {
        Asset(string)
    }
}