mod model;

use model::*;

use reqwest::{Client, Url};
use api::{Api, Asset, Market, Error};
use chrono::Utc;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use std::collections::{HashSet};
use stonks::Set;
use tokio_tungstenite::{WebSocketStream, self as tungstenite};
use tokio::net::TcpStream;

const ENDPOINT: &'static str = "https://www.binance.com/api/v3/";
const WS_ENDPOINT: &'static str = "wws://stream.binance.com:9443/ws";
const RECV_WINDOW: usize = 5000;

pub struct Binance<'a> {
    key: String,
    secret: String,
    client: Client,
    stream: WebSocketStream<TcpStream>,
    assets: Set<Asset>,
    markets: HashSet<Market<'a>>
}

impl<'a> Binance<'a> {
    pub async fn new() -> Self {
        Self {
            key: std::env::var("BINANCE_KEY").unwrap(),
            secret: std::env::var("BINANCE_SECRET").unwrap(),
            client: Client::builder().build().unwrap(),
            stream: tungstenite::connect_async(WS_ENDPOINT).await.unwrap().0,
            assets: Set::new(),
            markets: HashSet::new()
        }
    }
}

#[async_trait::async_trait]
impl<'a> Api<'a> for Binance<'a> {
    async fn update(&'a mut self) -> Result<(), Error> {
        let mut url = Url::parse(&format!("{}{}", ENDPOINT, "exchangeInfo")).unwrap();
        url.query_pairs_mut().append_pair("timestamp", &Utc::now().timestamp_millis().to_string());
        url.query_pairs_mut().append_pair("recvWindow", &RECV_WINDOW.to_string());

        let body = String::new();

        let mut mac = Hmac::<Sha256>::new_varkey(self.secret.as_bytes()).unwrap();
        let message = format!("{}{}", url.query().unwrap_or(""), body);
        mac.update(message.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());
        
        url.query_pairs_mut().append_pair("signature", &signature);

        let result = self.client
            .get(url)
            .header("X-MBX-APIKEY", &self.key)
            .body(body)
            .send()
            .await
            .unwrap()
            .json::<ExchangeInfo>()
            .await
            .unwrap();

        for (base, quote) in result.symbols.into_iter().map(|symbol| (symbol.base_asset.into(), symbol.quote_asset.into())) {
            let base = self.assets.get_or_insert(base);
            let quote = self.assets.get_or_insert(quote);

            let market = Market::from((base, quote));
            if !self.markets.contains(&market) {
                self.markets.insert(market);
            }
        }

        Ok(())
    }

    fn get_markets(&self) -> &HashSet<Market<'a>> {
        &self.markets
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
