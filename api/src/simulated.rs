use crate::{Api, Market, Candlestick, Order};

pub struct Simulated<API>
    where
        API: Api + Send + Sync + 'static
{
    api: API,
}

impl<API> Simulated<API>
    where
        API: Api + Send + Sync + 'static
{
} 

#[async_trait::async_trait]
impl<API> Api for Simulated<API>
    where
        API: Api + Send + Sync + 'static
{
    async fn get_markets<'a>(&mut self) -> Vec<Market<'a>> {
        self.api.get_markets().await
    }
    
    async fn next<'a>(&mut self, market: &Market<'a>) -> Candlestick<'a> {
        self.api.next(market).await
    }

    async fn last<'a>(&mut self, market: &Market<'a>) -> Candlestick<'a> {
        self.api.last(market).await
    }

    async fn order<'a>(&mut self, order: Order<'a>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}