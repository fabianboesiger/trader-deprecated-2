use crate::{Candlestick, Interval, Market};
use futures_core::{
    stream::Stream,
    task::{Context, Poll},
};
use std::fmt;
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio_tungstenite::{self as tungstenite, MaybeTlsStream, WebSocketStream};

pub struct Subscription<S>
where
    S: Stream<Item = Candlestick> + Unpin,
{
    pub market: &'static Market,
    pub interval: Interval,
    stream: S,
}

impl<S> Subscription<S>
where
    S: Stream<Item = Candlestick> + Unpin,
{
    pub fn new(market: &'static Market, interval: Interval, stream: S) -> Self {
        Self {
            market,
            interval,
            stream,
        }
    }
}

impl<S> fmt::Display for Subscription<S>
where
    S: Stream<Item = Candlestick> + Unpin,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.market, self.interval)
    }
}

impl<S> Stream for Subscription<S>
where
    S: Stream<Item = Candlestick> + Unpin,
{
    type Item = Candlestick;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.as_mut().get_mut().stream).poll_next(cx)
    }
}
