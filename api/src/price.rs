use crate::{Market, Monetary};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
pub struct Price {
    pub price: Monetary,
    pub market: &'static Market,
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.price.partial_cmp(&other.price)
    }
}

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        self.price.eq(&other.price)
    }
}
