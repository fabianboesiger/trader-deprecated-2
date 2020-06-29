use api::{Candlestick, Order};

#[async_trait::async_trait]
pub trait Trader {
    async fn evaluate<'a>(&mut self, candlestick: Candlestick<'a>) -> Option<Order>;
}

struct LuaTrader {
}

impl LuaTrader {
}

#[async_trait::async_trait]
impl Trader for LuaTrader {
    async fn evaluate<'a>(&mut self, candlestick: Candlestick<'a>) -> Option<Order> {
        let mut delay = time::delay_for(Duration::from_millis(50));
        tokio::select! {
            _ = delay => {
                println!("operation timed out");
            }
            _ = self.evaluate_lua() => {
                println!("operation completed");
            }
        }
    }
}