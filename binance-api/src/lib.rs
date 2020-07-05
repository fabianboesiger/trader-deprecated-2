mod model;

use api::{
    Api, Asset, Candlestick, Error, Interval, Market, Order, OrderError, OrderResponse, Price,
    Quantity, Subscription,
};
use chrono::Utc;
use futures::stream::{self, BoxStream, StreamExt};
use hmac::{Hmac, Mac, NewMac};
use reqwest::{Client, Url};
use sha2::Sha256;
use std::collections::HashSet;
use tokio_tungstenite::{self as tungstenite};
use serde::de::DeserializeOwned;
use std::fmt::Display;

const ENDPOINT: &'static str = "https://www.binance.com/api/v3/";
const WS_ENDPOINT: &'static str = "wss://stream.binance.com:9443/ws/";
const RECV_WINDOW: usize = 5000;

type SubscriptionStream = BoxStream<'static, Candlestick>;

macro_rules! params {
    ($($key:literal: $value:expr),*) => {
        {
            #[allow(unused_mut)]
            let mut params: Vec<(&'static str, Box<dyn Display + Send>)> = Vec::new();
            $(
                params.push(($key, Box::new($value)));
            )*
            params
        }
    }
}

pub struct Binance {
    key: String,
    secret: String,
    client: Client,
    assets: HashSet<&'static Asset>,
    markets: HashSet<&'static Market>,
}

impl Binance {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let key = std::env::var("BINANCE_API_KEY").expect("BINANCE_API_KEY environment variable not set");
        let secret = std::env::var("BINANCE_SECRET_KEY").expect("BINANCE_SECRET_KEY environment variable not set");

        Self {
            key,
            secret,
            client: Client::builder().build().unwrap(),
            assets: HashSet::new(),
            markets: HashSet::new(),
        }
    }

    async fn request<R>(&self, path: &'static str, params: Vec<(&'static str, Box<dyn Display + Send>)>) -> R
    where
        R: DeserializeOwned,
    {
        let mut url = Url::parse_with_params(
            &format!("{}{}", ENDPOINT, path), 
            params
                .into_iter()
                .map(|tuple| {
                    (tuple.0, format!("{}", tuple.1))
                })
                .collect::<Vec<(&str, String)>>()
        ).unwrap();

        /*
        url.query_pairs_mut()
            .append_pair("timestamp", &Utc::now().timestamp_millis().to_string());
        url.query_pairs_mut()
            .append_pair("recvWindow", &RECV_WINDOW.to_string());
        */

        let body = String::new();

        /*
        let mut mac = Hmac::<Sha256>::new_varkey(self.secret.as_bytes()).unwrap();
        let message = format!("{}{}", url.query().unwrap_or(""), body);
        mac.update(message.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        url.query_pairs_mut().append_pair("signature", &signature);
        */

        let result = self
            .client
            .get(url)
            .header("X-MBX-APIKEY", &self.key)
            .body(body)
            .send()
            .await
            .unwrap()
            .json::<R>()
            .await
            .unwrap();

        result
    }
}

#[async_trait::async_trait]
impl Api<SubscriptionStream> for Binance {
    async fn update(&mut self) -> Result<(), Error> {
        let result: model::ExchangeInfo = self.request("exchangeInfo", params!{}).await;

        for (base, quote) in result.symbols.into_iter().map(|symbol| {
            (
                Asset::from(symbol.base_asset),
                Asset::from(symbol.quote_asset),
            )
        }) {
            let base: &'static Asset = if let Some(base) = self.assets.get(&base) {
                base
            } else {
                let base: &'static Asset = Box::leak(Box::new(base.clone()));
                self.assets.insert(base);
                base
            };

            let quote: &'static Asset = if let Some(quote) = self.assets.get(&quote) {
                quote
            } else {
                let quote: &'static Asset = Box::leak(Box::new(quote.clone()));
                self.assets.insert(quote);
                quote
            };

            let market = Market::from((base, quote));
            if !self.markets.contains(&market) {
                let market: &'static Market = Box::leak(Box::new(market));
                self.markets.insert(market);
            }
        }

        Ok(())
    }

    fn get_markets(&self) -> &HashSet<&'static Market> {
        &self.markets
    }

    fn get_assets(&self) -> &HashSet<&'static Asset> {
        &self.assets
    }

    async fn subscribe(
        &self,
        market: &'static Market,
        interval: Interval,
    ) -> Subscription<SubscriptionStream> {
        let result: model::Candlesticks = self.request("klines", params!{
            "symbol": market,
            "interval": interval
        }).await;

        let candlestick = Candlestick {
            market,
            open_time: 0,
            close_time: 0,
            high: Price { price: 0.0, market },
            low: Price { price: 0.0, market },
            open: Price { price: 0.0, market },
            close: Price { price: 0.0, market },
            volume: Quantity {
                quantity: 0.0,
                asset: market.base,
            },
            trades: 0,
        };

        let buffer = vec![candlestick];
        
        let url = format!("{}{}@kline_{}", WS_ENDPOINT, market, interval);
        let live_stream = tungstenite::connect_async(url)
            .await
            .unwrap()
            .0
            .filter_map(move |result| async move { Some(candlestick) });
        let buffered_stream = stream::iter(buffer);
        let stream = buffered_stream
            .chain(live_stream)
            .map(|candlestick| {
                candlestick
            })
            .boxed();

        Subscription::new(market, interval, stream)
    }

    async fn order(&mut self, order: Order) -> Result<OrderResponse, OrderError> {
        //let result = self.request(path, params)
        Err(OrderError::Other(Error::ConnectionError))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_subscribe() {
        let mut binance = Binance::new();
        binance.update().await.unwrap();

        let market = *binance
            .get_markets()
            .iter()
            .find(|market| {
                format!("{}", market) == "ETHBTC"
            })
            .unwrap();
        
        println!("{}", market);
        let mut subscription = binance.subscribe(market, Interval::I5m).await;
        
        while let Some(candlestick) = subscription.next().await {
            println!("{:#?}", candlestick);
        }
    }
}
