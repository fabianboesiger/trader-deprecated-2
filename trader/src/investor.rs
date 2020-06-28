use api::{Api, Market};
use crate::{Trader, Indicator};

struct Investor<API>
    where
        API: Api
{
    markets: HashMap<MarketKey, 
    indicators: Vec<Box<dyn Indicator>>,
    traders: Vec<Box<dyn Trader>>,
}

impl<API> Investor<API>
    where
        API: Api 
{
    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                candlestick = self.api.next() => {
                    
                }
                _ = self.api.last() => {
    
                }
            }
        }
    }
}