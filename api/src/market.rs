use crate::{Asset, Filter};
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Market {
    pub base: &'static Asset,
    pub quote: &'static Asset,
    filters: Vec<Box<dyn Filter>>,
}

impl fmt::Display for Market {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.base, self.quote)
    }
}

impl PartialEq for Market {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.quote == other.quote
    }
}

impl Eq for Market {}

impl Hash for Market {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.hash(state);
        self.quote.hash(state);
    }
}

impl From<(&'static Asset, &'static Asset)> for Market {
    fn from((base, quote): (&'static Asset, &'static Asset)) -> Market {
        Market {
            base,
            quote,
            filters: Vec::new(),
        }
    }
}
