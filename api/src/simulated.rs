use crate::{Api, Market, Candlestick, Order, Error};
use std::collections::HashSet;

pub struct Simulated<'a, API>
    where
        API: Api<'a> + Send + Sync + 'static
{
    api: API,
    _phantom: std::marker::PhantomData<&'a API>
}

impl<'a, API> Simulated<'a, API>
    where
        API: Api<'a> + Send + Sync + 'static
{
} 

#[async_trait::async_trait]
impl<'a, API> Api<'a> for Simulated<'a, API>
    where
        API: Api<'a> + Send + Sync + 'static
{
    async fn update(&'a mut self) -> Result<(), Error> {
        self.api.update().await
    }

    fn get_markets(&self) -> &HashSet<Market<'a>> {
        self.api.get_markets()
    }
    
    async fn next(&mut self, market: &Market<'a>) -> Candlestick<'a> {
        self.api.next(market).await
    }

    async fn last(&mut self, market: &Market<'a>) -> Candlestick<'a> {
        self.api.last(market).await
    }

    async fn order(&mut self, order: Order<'a>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}