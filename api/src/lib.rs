mod asset;
mod candlestick;
mod error;
mod interval;
mod market;
mod price;
mod quantity;
mod simulated;
mod subscription;

pub use asset::*;
pub use candlestick::*;
pub use error::*;
pub use interval::*;
pub use market::*;
pub use price::*;
pub use quantity::*;
pub use simulated::*;
pub use subscription::*;

use futures_core::stream::Stream;
use std::collections::HashSet;
use std::fmt::Debug;

pub type Monetary = f64;

pub trait Filter: Debug + Send + Sync {
    fn apply(&self, order: Order) -> Result<Order, ()>;
}

#[derive(Copy, Clone)]
pub enum Side {
    Buy,
    Sell,
}

impl Side {
    pub fn reverse(&self) -> Self {
        match self {
            Buy => Self::Sell,
            Sell => Self::Buy,
        }
    }
}

pub enum Order {
    Limit(Side, Quantity, Price),
    //StopLoss(&'static Market, Quantity),
    //TakeProfit(&'static Market, Quantity),
    Oco(Side, Quantity, Price, Price),
}

/*
pub enum OrderState {
}

pub struct OrderWatcher {
    stream: Stream
}

impl OrderWatcher {
    pub async fn check() -> Result<OrderState, Error> {
        Err(Error::ConnectionError)
    }
}

pub enum PositionState {
    Entering(OrderState),
    Leaving(OrderState)
}

pub enum PositionOrderWatcher {
    Entering(OrderWatcher),
    Leaving(OrderWatcher)
}

pub struct PositionWatcher {
    stream: PositionOrderWatcher,
    take_profit: Price,
    stop_loss: Price,
}

impl PositionWatcher {
    pub(crate) fn new(order_watcher: OrderWatcher, take_profit: Price, stop_loss: Price) -> Self {
        Self {
            stream: PositionOrderWatcher::Entering(order_watcher),
            take_profit,
            stop_loss,
        }
    }

    pub async fn check() -> Result<PositionState, Error> {


        Err(Error::ConnectionError)
    }
}



impl From<Error> for PositionError {
    fn from(error: Error) -> Self {
        Self::Other(error)
    }
}

impl From<OrderError> for PositionError {
    fn from(error: OrderError) -> Self {
        Self::Order(error)
    }
}
*/

pub struct OrderResponse {
    pub executed_quantity: Quantity,
}

pub enum OrderError {
    Other(Error),
}

pub struct PositionResponse {
    pub executed_quantity: Quantity,
}

impl From<(OrderResponse, OrderResponse)> for PositionResponse {
    fn from((entering_response, leaving_response): (OrderResponse, OrderResponse)) -> Self {
        Self {
            executed_quantity: entering_response.executed_quantity,
        }
    }
}

pub enum PositionError {
    DifferentMarkets,
    PriceRestrictions,
    WrongAsset,
    Other(Error),
}

impl From<Error> for PositionError {
    fn from(error: Error) -> Self {
        Self::Other(error)
    }
}

impl From<OrderError> for PositionError {
    fn from(error: OrderError) -> Self {
        match error {
            OrderError::Other(error) => Self::Other(error),
        }
    }
}

#[async_trait::async_trait]
pub trait Api<S>
where
    S: Stream<Item = Candlestick> + Unpin + Send + 'static,
{
    const NAME: &'static str = "";

    /// Updates the API with the latest exchange information.
    async fn update(&mut self) -> Result<(), Error>;

    /// Returns all available Markets.
    fn get_markets(&self) -> &HashSet<&'static Market>;

    /// Returns all available Assets.
    fn get_assets(&self) -> &HashSet<&'static Asset>;

    /// Subscribe to market events.
    async fn subscribe(&self, market: &'static Market, interval: Interval) -> Subscription<S>;

    //async fn next(&mut self, market: &Market) -> Candlestick;
    //async fn get_markets<'a>(&mut self) -> Vec<Market<'a>>;
    //async fn get_previous_candlesticks<'a>(&mut self, market: &Market<'a>) -> Vec<Candlestick<'a>>;
    //async fn get_current_candlestick<'a>(&mut self, market: &Market<'a>) -> Candlestick<'a>;

    /// Create a new order.
    /// Instead of using this method directly, consider using the `enter_position` method instead
    /// as it handles things probably as needed.
    async fn order(&mut self, order: Order) -> Result<OrderResponse, OrderError>;

    /// This method is used to easily enter a position.
    async fn enter_position(
        &mut self,
        side: Side,
        enter_quantity: Quantity,
        enter_price: Price,
        take_profit: Price,
        stop_loss: Price,
    ) -> Result<PositionResponse, PositionError> {
        // Check if the same markets were chosen.
        if !(take_profit.market == enter_price.market && stop_loss.market == enter_price.market) {
            return Err(PositionError::DifferentMarkets);
        }

        // Check if the price restrictions apply.
        if !match side {
            Side::Buy => take_profit > enter_price && enter_price > stop_loss,
            Side::Sell => take_profit < enter_price && enter_price < stop_loss,
        } {
            return Err(PositionError::PriceRestrictions);
        }

        // Check if the correct asset was chosen.
        if !match side {
            Side::Buy => enter_quantity.asset == enter_price.market.base,
            Side::Sell => enter_quantity.asset == enter_price.market.quote,
        } {
            return Err(PositionError::WrongAsset);
        }

        let reverse = side.reverse();

        let entering_response = self
            .order(Order::Limit(side, enter_quantity, enter_price))
            .await?;
        let leaving_response = self
            .order(Order::Oco(
                reverse,
                entering_response.executed_quantity,
                stop_loss,
                take_profit,
            ))
            .await?;

        Ok(PositionResponse::from((
            entering_response,
            leaving_response,
        )))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
