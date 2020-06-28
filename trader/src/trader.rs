use api::{Candlestick, Order};

pub trait Trader {
    fn evaluate<'a>(&mut self, candlestick: Candlestick<'a>) -> Option<Order>;
}