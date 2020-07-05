use crate::{
    Api, Asset, Candlestick, Error, Interval, Market, Order, OrderError, OrderResponse,
    Subscription,
};
use futures_core::stream::Stream;
use std::collections::HashSet;

pub struct Simulated<API, S>
where
    API: Api<S> + Send + Sync + 'static,
    S: Stream<Item = Candlestick> + Unpin + Send + Sync + 'static,
{
    api: API,
    _phantom: std::marker::PhantomData<S>,
}

impl<API, S> Simulated<API, S>
where
    API: Api<S> + Send + Sync + 'static,
    S: Stream<Item = Candlestick> + Unpin + Send + Sync + 'static,
{
}

#[async_trait::async_trait]
impl<API, S> Api<S> for Simulated<API, S>
where
    API: Api<S> + Send + Sync + 'static,
    S: Stream<Item = Candlestick> + Unpin + Send + Sync + 'static,
{
    async fn update(&mut self) -> Result<(), Error> {
        self.api.update().await
    }

    fn get_markets(&self) -> &HashSet<&'static Market> {
        self.api.get_markets()
    }

    fn get_assets(&self) -> &HashSet<&'static Asset> {
        self.api.get_assets()
    }

    async fn subscribe(&self, market: &'static Market, interval: Interval) -> Subscription<S> {
        self.api.subscribe(market, interval).await
    }

    async fn order(&mut self, order: Order) -> Result<OrderResponse, OrderError> {
        Err(OrderError::Other(Error::ConnectionError))
    }
}
