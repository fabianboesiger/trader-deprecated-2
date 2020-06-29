use crate::{Asset, Filter};
use std::hash::{Hash, Hasher};

pub struct Market<'a> {
    pub base: &'a Asset,
    pub quote: &'a Asset,
    filters: Vec<Box<dyn Filter>>
}

impl<'a> PartialEq for Market<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.quote == other.quote
    }
}

impl<'a> Eq for Market<'a> {}

impl<'a> Hash for Market<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.hash(state);
        self.quote.hash(state);
    }
}

impl<'a> From<(&'a Asset, &'a Asset)> for Market<'a> {
    fn from((base, quote): (&'a Asset, &'a Asset)) -> Market<'a> {
        Market {
            base,
            quote,
            filters: Vec::new()
        }
    }
}