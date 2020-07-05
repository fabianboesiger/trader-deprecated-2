use crate::{Error, Market, Price, Quantity};

#[derive(Debug, Copy, Clone)]
pub struct Candlestick {
    pub market: &'static Market,
    pub open_time: u64,
    pub close_time: u64,
    pub high: Price,
    pub low: Price,
    pub open: Price,
    pub close: Price,
    pub volume: Quantity,
    pub trades: u64,
}

impl Candlestick {
    pub fn insert(&self) -> Result<(), Error> {
        Ok(())
    }
}
