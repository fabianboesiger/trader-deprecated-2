mod simulated;
mod asset;
mod quantity;
mod market;

pub use asset::*;
pub use quantity::*;
pub use simulated::*;
pub use market::*;

pub type Monetary = f64;

#[derive(Copy, Clone)]
pub struct Price<'a> {
    price: Monetary,
    market: &'a Market<'a>
}

pub trait Filter: Send + Sync {
    fn apply(&self, order: Order) -> Result<Order, ()>;
}


pub struct Candlestick<'a> {
    market: &'a Market<'a>,
    open_time: u64,
    close_time: u64,
    high: Price<'a>,
    low: Price<'a>,
    open: Price<'a>,
    close: Price<'a>,
    volume: Quantity<'a>,
    trades: u64,
}

pub enum Order<'a> {
    Limit(Price<'a>, Quantity<'a>),
    StopLoss(&'a Market<'a>, Quantity<'a>),
    TakeProfit(&'a Market<'a>, Quantity<'a>)
}

#[async_trait::async_trait]
pub trait Api {
    const NAME: &'static str = "";
    
    async fn get_markets<'a>(&mut self) -> Vec<Market<'a>>;
    async fn next<'a>(&mut self, market: &Market<'a>) -> Candlestick<'a>;
    async fn last<'a>(&mut self, market: &Market<'a>) -> Candlestick<'a>;
    //async fn get_markets<'a>(&mut self) -> Vec<Market<'a>>;
    //async fn get_previous_candlesticks<'a>(&mut self, market: &Market<'a>) -> Vec<Candlestick<'a>>;
    //async fn get_current_candlestick<'a>(&mut self, market: &Market<'a>) -> Candlestick<'a>;
    async fn order<'a>(&mut self, order: Order<'a>) -> Result<(), Box<dyn std::error::Error>>;
    async fn order_window<'a>(&mut self, price: Price<'a>, quantity: Quantity<'a>, stop_loss: Quantity<'a>, take_profit: Quantity<'a>) -> Result<(), Box<dyn std::error::Error>> {
        self.order(Order::Limit(price, quantity)).await?;
        self.order(Order::StopLoss(price.market, stop_loss)).await?;
        self.order(Order::TakeProfit(price.market, take_profit)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
