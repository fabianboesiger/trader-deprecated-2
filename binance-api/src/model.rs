use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub code: i64,
    pub msg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    //pub timezone: String,
    //pub server_time: u64,
    //pub rate_limits: Vec<RateLimit>,
    //pub exchange_filters: Vec<ExchangeFilter>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u64,
    pub quote_asset: String,
    pub quote_precision: u64,
    pub order_types: Vec<String>,
    pub iceberg_allowed: bool,
    pub filters: Vec<SymbolFilter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolFilter {
    #[serde(rename_all = "camelCase")]
    LotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        min_price: String,
        max_price: String,
        tick_size: String,
    },
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        multiplier_up: String,
        multiplier_down: String,
        avg_price_mins: u64,
    },
    #[serde(rename_all = "camelCase")]
    MinNotional { min_notional: String },
    #[serde(rename_all = "camelCase")]
    MaxPosition { max_position: String },
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { max_num_algo_orders: u64 },
    #[serde(rename_all = "camelCase")]
    MaxNumIcebergOrders { max_num_iceberg_orders: u64 },
    #[serde(rename_all = "camelCase")]
    MaxNumOrders { max_num_orders: u64 },
    #[serde(rename_all = "camelCase")]
    IcebergParts { limit: u64 },
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Value {
    Integer(i64),
    Float(String)
}

impl Value {
    pub fn as_u64(&self) -> Option<u64> {
        if let Self::Integer(value) = self {
            Some(*value as u64)
        } else {
            None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        if let Self::Float(value) = self {
            if let Ok(value) = value.parse::<f64>() {
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub type IntoCandlestick = Vec<Value>;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(from = "IntoCandlestick")]
pub struct Candlestick {
    pub open_time: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: u64,
    pub quote_asset_volume: f64,
    pub number_of_trades: u64,
    pub taker_buy_base_asset_volume: f64,
    pub taker_buy_quote_asset_volume: f64,
}

impl From<IntoCandlestick> for Candlestick {
    fn from(from: IntoCandlestick) -> Candlestick {
        Candlestick {
            open_time: from[0].as_u64().unwrap(),
            open: from[1].as_f64().unwrap(),
            high: from[2].as_f64().unwrap(),
            low: from[3].as_f64().unwrap(),
            close: from[4].as_f64().unwrap(),
            volume: from[5].as_f64().unwrap(),
            close_time: from[6].as_u64().unwrap(),
            quote_asset_volume: from[7].as_f64().unwrap(),
            number_of_trades: from[8].as_u64().unwrap(),
            taker_buy_base_asset_volume: from[9].as_f64().unwrap(),
            taker_buy_quote_asset_volume: from[10].as_f64().unwrap(),
        }
    }
}

pub type Candlesticks = Vec<Candlestick>;


mod string_or_float {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}